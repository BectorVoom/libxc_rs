//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1681/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1681(t15101: f64, t23767: f64, t15421: f64, t23770: f64, t11299: f64, t6141: f64, t6145: f64, t11450: f64, t11466: f64, t11507: f64, t15104: f64, t15350: f64, t15406: f64, t1633: f64, t1634: f64, t23694: f64, t23723: f64, t23758: f64, t23764: f64, t23773: f64, t23785: f64, t2968: f64, t2970: f64, t2987: f64, t3012: f64, t3014: f64, t311: f64, t52443: f64, t52812: f64, t6158: f64, t6173: f64, t6189: f64, t6190: f64, t6205: f64, t6209: f64, t63997: f64, t64043: f64, t78207: f64, t88008: f64, t88055: f64, t88510: f64, t88524: f64, t88537: f64) -> (f64, f64, f64, f64) {
    let t88562 = 24.0_f64 * t15101 * t23767;
    let t88564 = 0.1929837539843104208e3_f64 * t15421 * t23770;
    let t88567 = 0.57895126195293126241e3_f64 * t11299 * t6145 * t6141;
    let t88570 = -0.46785788981077169656e1_f64 * t2987 * t1634 * t23694 - 24.0_f64 * t15104 * t23773 + 0.61524113149298439947e4_f64 * t11507 * t64043 * t6189 - t88510 + 36.0_f64 * t2968 * t6158 * t6173 - 0.310907e-1_f64 * (t88524 + t88537) * t311 - 0.62337092780453269531e3_f64 * t11466 * t6209 * t6205 + 0.2077903092681775651e3_f64 * t15350 * t23764 + 0.69263436422725855036e2_f64 * t3012 * t78207 * t1633 - 0.77193501593724168322e3_f64 * t52812 * t23723 + 0.11579025239058625248e4_f64 * t11450 * t88055 * t2970 - 0.70178683471615754484e1_f64 * t63997 * t6190 - 0.4155806185363551302e3_f64 * t52443 * t23785 + 0.6233709278045326953e3_f64 * t11507 * t88008 * t3014 + t88562 - t88564 + t88567 + 0.3859675079686208416e3_f64 * t15406 * t23758;
    (t88562, t88564, t88567, t88570)
}
