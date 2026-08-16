//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1031/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1031(t11120: f64, t318: f64, t294: f64, t2814: f64, t4019: f64, t11004: f64, t10982: f64, t10980: f64, t10986: f64, t11002: f64, t11010: f64, t11015: f64, t11020: f64, t11024: f64, t11028: f64, t11033: f64, t11037: f64, t8605: f64, t8607: f64, t8616: f64, t8618: f64, t8723: f64) -> (f64, f64, f64, f64) {
    let t11121 = t11120 * t318;
    let t11123 = 0.19751673498613801407e-1_f64 * t294 * t11121;
    let t11124 = t4019 * t2814;
    let t11134 = 0.23744444444444444444e-1_f64 * t11004;
    let t11135 = 0.11872222222222222222e-1_f64 * t10982;
    let t11144 = -t8723 - 0.15829629629629629629e-1_f64 * t8616 + 0.39574074074074074073e-2_f64 * t8607 - 0.11872222222222222222e-1_f64 * t8618 + 0.5936111111111111111e-2_f64 * t8605 - 0.79148148148148148146e-2_f64 * t10980 + 0.79148148148148148146e-2_f64 * t11002 - t11134 + t11135 - 0.19787037037037037037e-1_f64 * t11010 + 0.71233333333333333332e-1_f64 * t11015 - 0.23744444444444444444e-1_f64 * t11020 - 0.11872222222222222222e-1_f64 * t11024 - 0.10685e0_f64 * t11028 + 0.71233333333333333332e-1_f64 * t11033 + 0.35616666666666666666e-1_f64 * t11037 - 0.17808333333333333333e-1_f64 * t10986;
    (t11121, t11123, t11124, t11144)
}
