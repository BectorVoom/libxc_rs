//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1109/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1109<F: Float>(t1542: F, t9176: F, t2975: F, t1531: F, t2931: F, t11875: F, t11942: F, t11873: F, t11880: F, t11885: F, t11890: F, t11896: F, t11899: F, t11904: F, t11908: F, t11938: F, t11952: F, t9221: F, t9223: F, t9226: F, t9228: F, t9399: F) -> (F, F, F) {
    let t12218 = t1542 * t9176;
    let t12219 = t12218 * t2975;
    let t12222 = t1531 * t2931;
    let t12231 = F::cast_from(0.23744444444444444444e-1_f64) * t11875;
    let t12232 = F::cast_from(0.11872222222222222222e-1_f64) * t11942;
    let t12241 = -t9399 + F::cast_from(0.15829629629629629629e-1_f64) * t9221 + F::cast_from(0.39574074074074074073e-2_f64) * t9223 - F::cast_from(0.11872222222222222222e-1_f64) * t9226 - F::cast_from(0.5936111111111111111e-2_f64) * t9228 + F::cast_from(0.79148148148148148146e-2_f64) * t11938 + F::cast_from(0.79148148148148148146e-2_f64) * t11873 - t12231 - t12232 + F::cast_from(0.19787037037037037037e-1_f64) * t11880 - F::cast_from(0.71233333333333333332e-1_f64) * t11885 - F::cast_from(0.23744444444444444444e-1_f64) * t11890 - F::cast_from(0.11872222222222222222e-1_f64) * t11896 + F::cast_from(0.10685e0_f64) * t11899 + F::cast_from(0.71233333333333333332e-1_f64) * t11904 + F::cast_from(0.35616666666666666666e-1_f64) * t11908 + F::cast_from(0.17808333333333333333e-1_f64) * t11952;
    (t12219, t12222, t12241)
}
