//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1643/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1643(t12571: f64, t3535: f64, t1196: f64, t3516: f64, t3542: f64, t43830: f64, t43832: f64, t43837: f64, t43841: f64, t43845: f64, t43849: f64, t43858: f64, t43862: f64, t43865: f64, t43871: f64, t43877: f64) -> (f64, f64, f64) {
    let t44984 = 0.70178683471615754484e1_f64 * t12571 * t3535;
    let t44987 = 0.21053605041484726346e2_f64 * t1196 * t3542 * t3516;
    let t44999 = -0.26382716049382716049e-1_f64 * t43858 - 0.52765432098765432099e-1_f64 * t43862 - 0.14246666666666666667e0_f64 * t43830 - 0.31659259259259259258e-1_f64 * t43865 + 0.47488888888888888888e-1_f64 * t43832 + 0.11872222222222222222e0_f64 * t43837 - 0.35616666666666666666e-1_f64 * t43871 - 0.47488888888888888888e-1_f64 * t43841 + 0.6411e0_f64 * t43845 + 0.10685e0_f64 * t43877 + 0.14246666666666666667e0_f64 * t43849;
    (t44984, t44987, t44999)
}
