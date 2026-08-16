//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1681/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1681<F: Float>(t15101: F, t23767: F, t15421: F, t23770: F, t11299: F, t6141: F, t6145: F, t11450: F, t11466: F, t11507: F, t15104: F, t15350: F, t15406: F, t1633: F, t1634: F, t23694: F, t23723: F, t23758: F, t23764: F, t23773: F, t23785: F, t2968: F, t2970: F, t2987: F, t3012: F, t3014: F, t311: F, t52443: F, t52812: F, t6158: F, t6173: F, t6189: F, t6190: F, t6205: F, t6209: F, t63997: F, t64043: F, t78207: F, t88008: F, t88055: F, t88510: F, t88524: F, t88537: F) -> (F, F, F, F) {
    let t88562 = F::cast_from(24.0_f64) * t15101 * t23767;
    let t88564 = F::cast_from(0.1929837539843104208e3_f64) * t15421 * t23770;
    let t88567 = F::cast_from(0.57895126195293126241e3_f64) * t11299 * t6145 * t6141;
    let t88570 = -F::cast_from(0.46785788981077169656e1_f64) * t2987 * t1634 * t23694 - F::cast_from(24.0_f64) * t15104 * t23773 + F::cast_from(0.61524113149298439947e4_f64) * t11507 * t64043 * t6189 - t88510 + F::cast_from(36.0_f64) * t2968 * t6158 * t6173 - F::cast_from(0.310907e-1_f64) * (t88524 + t88537) * t311 - F::cast_from(0.62337092780453269531e3_f64) * t11466 * t6209 * t6205 + F::cast_from(0.2077903092681775651e3_f64) * t15350 * t23764 + F::cast_from(0.69263436422725855036e2_f64) * t3012 * t78207 * t1633 - F::cast_from(0.77193501593724168322e3_f64) * t52812 * t23723 + F::cast_from(0.11579025239058625248e4_f64) * t11450 * t88055 * t2970 - F::cast_from(0.70178683471615754484e1_f64) * t63997 * t6190 - F::cast_from(0.4155806185363551302e3_f64) * t52443 * t23785 + F::cast_from(0.6233709278045326953e3_f64) * t11507 * t88008 * t3014 + t88562 - t88564 + t88567 + F::cast_from(0.3859675079686208416e3_f64) * t15406 * t23758;
    (t88562, t88564, t88567, t88570)
}
