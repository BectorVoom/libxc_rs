//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 1287/1328 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk1287<F: Float>(t35759: F, t35762: F, t35764: F, t35768: F, t35772: F, t35776: F, t35780: F, t35783: F, t35788: F, t35792: F, t35795: F, t35798: F, t35801: F) -> F {
    let t37524 = -F::new(0.68394856556563412154e-6) * t35759 - F::new(0.39896999657995323756e-6) * t35762 + F::new(0.64586396578113893434e-4) * t35764 - F::new(0.42812952277158830873e-7) * t35768 - F::new(0.13678971311312682431e-5) * t35772 - F::new(0.34798367610874697734e-6) * t35776 - F::new(0.34798367610874697734e-6) * t35780 - F::new(0.58714905980103539484e-5) * t35783 - F::new(0.98994396179416340121e-6) * t35788 - F::new(0.58714905980103539484e-5) * t35792 - F::new(0.58714905980103539484e-5) * t35795 - F::new(0.29357452990051769742e-5) * t35798 + F::new(0.34798367610874697734e-6) * t35801;
    t37524
}
