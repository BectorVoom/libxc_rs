//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 744/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk744<F: Float>(t3879: F, t539: F, t1373: F, t225: F, t1376: F, t566: F, t68: F, t1385: F, t3787: F, t562: F, t3793: F, t1338: F, t1372: F) -> (F, F, F, F, F, F, F, F) {
    let t3880 = t539 * t3879;
    let t3882 = t1373 * t225;
    let t3886 = F::cast_from(1.0_f64) / t1376 / t566;
    let t3887 = t68 * t3886;
    let t3888 = t1385 * t1385;
    let t3889 = t3887 * t3888;
    let t3897 = t3787 * t562;
    let t3898 = t3897 * t3793;
    let t3901 = t1338 * t1372;
    (t3880, t3882, t3887, t3888, t3889, t3897, t3898, t3901)
}
