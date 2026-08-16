//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1470/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1470(t11399: f64, t11411: f64, t11444: f64, t11445: f64, t11450: f64, t11466: f64, t11468: f64, t11501: f64, t11507: f64, t11513: f64, t11548: f64, t2938: f64, t2943: f64, t2945: f64, t2962: f64, t2963: f64, t2968: f64, t2970: f64, t2971: f64, t2987: f64, t2988: f64, t2989: f64, t3006: f64, t3012: f64, t3014: f64, t3015: f64, t41225: f64, t41513: f64, t41668: f64, t41779: f64, t41785: f64, t41788: f64, t41794: f64, t41799: f64, t41813: f64, t955: f64, t974: f64) -> f64 {
    let t41825 = -0.77193501593724168322e3_f64 * t41779 * t11411 + 0.11579025239058625248e4_f64 * t11450 * t41668 * t2970 - 0.70178683471615754484e1_f64 * t41785 * t2989 - 0.4155806185363551302e3_f64 * t41788 * t11468 + 0.6233709278045326953e3_f64 * t11507 * t41225 * t3014 + 4.0_f64 * t41794 * t955 + 6.0_f64 * t11399 * t2963 + 0.1929837539843104208e3_f64 * t41799 * t2971 + 4.0_f64 * t2938 * t11445 + 0.21053605041484726346e2_f64 * t3012 * t2989 * t3006 - 0.46785788981077169656e1_f64 * t2987 * t974 * t11501 - 0.62337092780453269531e3_f64 * t11466 * t3015 * t3006 - t41513 + 0.61524113149298439947e4_f64 * t11507 * t41813 * t2988 - 24.0_f64 * t11548 * t11513 + 36.0_f64 * t2968 * t2945 * t2962 - 8.0_f64 * t2943 * t955 * t11444;
    t41825
}
