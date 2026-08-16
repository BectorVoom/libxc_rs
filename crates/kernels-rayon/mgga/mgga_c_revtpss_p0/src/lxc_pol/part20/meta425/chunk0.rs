//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1593/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1593(t1196: f64, t12552: f64, t3497: f64, t43977: f64, t12235: f64, t3531: f64, t43830: f64, t43832: f64, t43837: f64, t43841: f64, t43845: f64, t43849: f64, t43858: f64, t43862: f64, t43865: f64, t43871: f64, t43877: f64) -> (f64, f64, f64) {
    let t43980 = 0.61524113149298439947e4_f64 * t1196 * t12552 * t3497 * t43977;
    let t43982 = 0.14035736694323150897e2_f64 * t3531 * t12235;
    let t43994 = -0.13734567901234567901e-1_f64 * t43858 - 0.27469135802469135803e-1_f64 * t43862 - 0.74166666666666666668e-1_f64 * t43830 - 0.16481481481481481482e-1_f64 * t43865 + 0.24722222222222222222e-1_f64 * t43832 + 0.61805555555555555555e-1_f64 * t43837 - 0.18541666666666666666e-1_f64 * t43871 - 0.24722222222222222222e-1_f64 * t43841 + 0.33375e0_f64 * t43845 + 0.55625000000000000001e-1_f64 * t43877 + 0.74166666666666666668e-1_f64 * t43849;
    (t43980, t43982, t43994)
}
