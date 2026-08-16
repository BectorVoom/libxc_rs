//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 1814/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1814(t18637: f64, t2747: f64, t4365: f64, t10779: f64, t14671: f64, t6035: f64, t10777: f64, t14676: f64, t18444: f64, t4364: f64, t837: f64, t14894: f64, t14907: f64, t14925: f64, t14934: f64, t18527: f64, t18532: f64, t18618: f64, t18623: f64, t18629: f64, t18634: f64, t2745: f64, t4362: f64, t825: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t18639 = t2747 * t4365 * t18637;
    let t18643 = t10779 * t14671 * t6035;
    let t18644 = t10777 * t18643;
    let t18647 = t2747 * t14676 * t6035;
    let t18651 = t4364 * t18444 * t837;
    let t18654 = -0.12862205435420921092e-2_f64 * t14894 * t18527 - 0.12705000702321332056e-4_f64 * t18532 - 0.21437009059034868486e-3_f64 * t825 * t18618 - 0.12705000702321332056e-4_f64 * t18623 - 0.80031500487063509015e-2_f64 * t14907 - t14925 + 0.50820002809285328224e-4_f64 * t14934 + 0.85748036236139473944e-3_f64 * t2745 * t18629 + 0.85748036236139473944e-3_f64 * t4362 * t18634 + 0.17149607247227894789e-2_f64 * t2745 * t18639 + 0.10164000561857065645e-3_f64 * t18644 + 0.17149607247227894789e-2_f64 * t2745 * t18647 - 0.21437009059034868486e-3_f64 * t2745 * t18651;
    (t18639, t18643, t18644, t18647, t18651, t18654)
}
