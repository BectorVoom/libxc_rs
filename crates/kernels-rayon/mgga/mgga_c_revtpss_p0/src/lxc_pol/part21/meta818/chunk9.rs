//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3020/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3020(t11951: f64, t4858: f64, t11922: f64, t15906: f64, t15909: f64, t16067: f64, t16069: f64, t11200: f64, t380: f64, t16088: f64, t1025: f64, t11623: f64, t11783: f64, t15651: f64, t15717: f64, t15780: f64, t15785: f64, t15895: f64, t16017: f64, t16049: f64, t3092: f64, t3117: f64, t3224: f64, t371: f64, t372: f64, t373: f64, t42765: f64, t4854: f64, t4879: f64, t4892: f64, t53273: f64, t906: f64) -> (f64, f64) {
    let t55320 = t4858 * t11951;
    let t55325 = t15906 * t11922 * t15909;
    let t55328 = t16067 * t11922 * t16069;
    let t55330 = t11200 * t380;
    let t55331 = t55330 * t16088;
    let t55338 = 0.12862205435420921092e-2_f64 * t4892 * t3117 * t15780 * t15785 + 0.21437009059034868486e-3_f64 * t4879 * t11623 - 0.64311027177104605458e-3_f64 * t11783 * t4854 - 0.64311027177104605458e-3_f64 * t3224 * t15651 - 0.21437009059034868486e-3_f64 * t1025 * t371 * t372 * t373 * t53273 - 0.42874018118069736972e-3_f64 * t55320 + 0.68598428988911579154e-2_f64 * t16049 * t16017 - 0.25724410870841842183e-2_f64 * t55325 + 0.42874018118069736972e-3_f64 * t55328 - 0.25724410870841842184e-2_f64 * t55331 * t3092 * t15717 * t906 + 0.68598428988911579154e-2_f64 * t42765 * t15895;
    (t55330, t55338)
}
