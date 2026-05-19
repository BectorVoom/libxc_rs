//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1198/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1198<F: Float>(t1559: F, t1564: F, t169: F, t31548: F, t32033: F, t10265: F, t3833: F, t1063: F, t2440: F, t7974: F, t3818: F, t3344: F, t6338: F) -> (F, F, F, F, F) {
    let t32036 = F::cast_from(0.34146007962811379518e0_f64) * t31548 * t1559 * t169 * t1564 * t32033;
    let t32038 = F::cast_from(0.17073003981405689759e0_f64) * t3833 * t10265;
    let t32041 = F::cast_from(0.56910013271352299198e-1_f64) * t1063 * t2440 * t7974;
    let t32043 = F::cast_from(0.2276400530854091968e0_f64) * t3818 * t10265;
    let t32044 = t6338 * t3344;
    (t32036, t32038, t32041, t32043, t32044)
}
