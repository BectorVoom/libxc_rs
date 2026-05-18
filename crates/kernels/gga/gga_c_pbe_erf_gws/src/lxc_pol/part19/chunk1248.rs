//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1248/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1248<F: Float>(t51967: F, t4135: F, t51966: F, t1146: F, t13987: F, t13953: F, t3070: F, t13808: F, t14584: F, t4130: F, t51650: F, t13893: F, t4150: F) -> (F, F, F, F, F, F, F) {
    let t54619 = F::new(35.0) / F::new(216.0) * t51967;
    let t54621 = t51966 * t4135;
    let t54641 = t13987 * t1146;
    let t54681 = t13953 * t3070;
    let t54716 = t13808 * t14584;
    let t54719 = t51650 * t4130;
    let t54724 = t13893 * t4150;
    (t54619, t54621, t54641, t54681, t54716, t54719, t54724)
}
