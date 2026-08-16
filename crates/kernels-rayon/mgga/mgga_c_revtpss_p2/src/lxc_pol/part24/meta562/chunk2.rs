//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1692/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1692(t6258: f64, t6305: f64, t1651: f64, t23598: f64, t15962: f64, t5819: f64, t11704: f64, t1063: f64, t11257: f64, t11703: f64, t11853: f64, t11875: f64, t15618: f64, t19501: f64, t19611: f64, t19878: f64, t23470: f64, t23474: f64, t23911: f64, t23917: f64, t23966: f64, t24013: f64, t247: f64, t3091: f64, t3092: f64, t3116: f64, t3117: f64, t3162: f64, t3182: f64, t42410: f64, t42690: f64, t4837: f64, t4899: f64, t54570: f64, t78805: f64, t78855: f64, t88112: f64, t88128: f64, t88794: f64) -> (f64, f64, f64, f64, f64) {
    let t88804 = t6258 * t6305;
    let t88815 = t1651 * t23598;
    let t88828 = t15962 * t5819;
    let t88844 = t11704 * t5819;
    let t88849 = 0.25724410870841842184e-2_f64 * t54570 * t24013 - 0.17149607247227894789e-2_f64 * t78805 + 0.12862205435420921092e-2_f64 * t11875 * t3117 * t88804 * t3162 - 0.85748036236139473944e-3_f64 * t42690 * t3117 * t88794 * t11257 + 0.51448821741683684368e-2_f64 * t19878 * t23966 + 0.17149607247227894789e-2_f64 * t4837 * t247 * t3116 * t88815 + 0.71456696863449561621e-3_f64 * t1063 * t247 * t3182 * t88128 - 0.76220476654346199062e-2_f64 * t1063 * t247 * t11853 * t88112 + 0.17149607247227894789e-2_f64 * t4899 * t3092 * t19501 * t88828 + 0.2540682555144873302e-2_f64 * t3091 * t42410 * t23470 * t23911 + 0.28582678745379824648e-2_f64 * t15618 * t23917 + 0.19055119163586549765e-2_f64 * t78855 + 0.57165357490759649296e-3_f64 * t3091 * t3092 * t23474 * t23911 + 0.14291339372689912324e-2_f64 * t3091 * t11703 * t19611 * t88844;
    (t88804, t88815, t88828, t88844, t88849)
}
