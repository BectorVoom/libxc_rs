//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1692/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1692<F: Float>(t6258: F, t6305: F, t1651: F, t23598: F, t15962: F, t5819: F, t11704: F, t1063: F, t11257: F, t11703: F, t11853: F, t11875: F, t15618: F, t19501: F, t19611: F, t19878: F, t23470: F, t23474: F, t23911: F, t23917: F, t23966: F, t24013: F, t247: F, t3091: F, t3092: F, t3116: F, t3117: F, t3162: F, t3182: F, t42410: F, t42690: F, t4837: F, t4899: F, t54570: F, t78805: F, t78855: F, t88112: F, t88128: F, t88794: F) -> (F, F, F, F, F) {
    let t88804 = t6258 * t6305;
    let t88815 = t1651 * t23598;
    let t88828 = t15962 * t5819;
    let t88844 = t11704 * t5819;
    let t88849 = F::cast_from(0.25724410870841842184e-2_f64) * t54570 * t24013 - F::cast_from(0.17149607247227894789e-2_f64) * t78805 + F::cast_from(0.12862205435420921092e-2_f64) * t11875 * t3117 * t88804 * t3162 - F::cast_from(0.85748036236139473944e-3_f64) * t42690 * t3117 * t88794 * t11257 + F::cast_from(0.51448821741683684368e-2_f64) * t19878 * t23966 + F::cast_from(0.17149607247227894789e-2_f64) * t4837 * t247 * t3116 * t88815 + F::cast_from(0.71456696863449561621e-3_f64) * t1063 * t247 * t3182 * t88128 - F::cast_from(0.76220476654346199062e-2_f64) * t1063 * t247 * t11853 * t88112 + F::cast_from(0.17149607247227894789e-2_f64) * t4899 * t3092 * t19501 * t88828 + F::cast_from(0.2540682555144873302e-2_f64) * t3091 * t42410 * t23470 * t23911 + F::cast_from(0.28582678745379824648e-2_f64) * t15618 * t23917 + F::cast_from(0.19055119163586549765e-2_f64) * t78855 + F::cast_from(0.57165357490759649296e-3_f64) * t3091 * t3092 * t23474 * t23911 + F::cast_from(0.14291339372689912324e-2_f64) * t3091 * t11703 * t19611 * t88844;
    (t88804, t88815, t88828, t88844, t88849)
}
