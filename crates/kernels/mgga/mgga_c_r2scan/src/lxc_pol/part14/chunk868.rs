//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 868/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk868<F: Float>(t2060: F, t2482: F, t2062: F, t2823: F, t5998: F, t6027: F, t897: F, t6029: F, t4827: F, t4839: F, t4996: F, t5000: F, t5004: F, t5008: F, t7015: F, t7870: F) -> F {
    let t7872 = t2060 * t2482;
    let t7874 = F::new(0.1350520664e0) * t7872 * t2062;
    let t7876 = F::new(0.1350520664e0) * t2823 * t5998;
    let t7877 = t6027 * t897;
    let t7878 = t7877 * t6029;
    let t7880 = -F::new(0.675260332e-1) * t7870 - t7874 - t7876 + F::new(0.1350520664e0) * t7878 - t4996 + t5000 + t5004 + t5008 + t7015 + t4827 - t4839;
    t7880
}
