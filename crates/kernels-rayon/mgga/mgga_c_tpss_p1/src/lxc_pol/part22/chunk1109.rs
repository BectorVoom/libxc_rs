//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1109/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1109(t1542: f64, t9176: f64, t2975: f64, t1531: f64, t2931: f64, t11875: f64, t11942: f64, t11873: f64, t11880: f64, t11885: f64, t11890: f64, t11896: f64, t11899: f64, t11904: f64, t11908: f64, t11938: f64, t11952: f64, t9221: f64, t9223: f64, t9226: f64, t9228: f64, t9399: f64) -> (f64, f64, f64) {
    let t12218 = t1542 * t9176;
    let t12219 = t12218 * t2975;
    let t12222 = t1531 * t2931;
    let t12231 = 0.23744444444444444444e-1_f64 * t11875;
    let t12232 = 0.11872222222222222222e-1_f64 * t11942;
    let t12241 = -t9399 + 0.15829629629629629629e-1_f64 * t9221 + 0.39574074074074074073e-2_f64 * t9223 - 0.11872222222222222222e-1_f64 * t9226 - 0.5936111111111111111e-2_f64 * t9228 + 0.79148148148148148146e-2_f64 * t11938 + 0.79148148148148148146e-2_f64 * t11873 - t12231 - t12232 + 0.19787037037037037037e-1_f64 * t11880 - 0.71233333333333333332e-1_f64 * t11885 - 0.23744444444444444444e-1_f64 * t11890 - 0.11872222222222222222e-1_f64 * t11896 + 0.10685e0_f64 * t11899 + 0.71233333333333333332e-1_f64 * t11904 + 0.35616666666666666666e-1_f64 * t11908 + 0.17808333333333333333e-1_f64 * t11952;
    (t12219, t12222, t12241)
}
