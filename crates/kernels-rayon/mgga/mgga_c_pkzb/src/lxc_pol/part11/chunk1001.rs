//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1001/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1001(t10767: f64, t179: f64, t780: f64, t2946: f64, t3515: f64, t758: f64, t10934: f64, t10938: f64, t10945: f64, t10982: f64, t10986: f64, t10990: f64, t10995: f64, t2887: f64, t2899: f64, t2945: f64, t299: f64, t5591: f64, t5614: f64, t5725: f64, t5933: f64, t5954: f64, t757: f64, t7582: f64, t7621: f64, t7756: f64, t9308: f64) -> (f64, f64, f64, f64) {
    let t10999 = t179 * t780 * t10767;
    let t11004 = t2946 * t3515;
    let t11005 = t758 * t11004;
    let t11008 = -0.14291339372689912324e-3_f64 * t7582 + t5591 + t5614 - 0.51448821741683684368e-2_f64 * t299 * t10934 + t2887 * t10938 / 16.0_f64 + t7621 / 144.0_f64 + 0.21437009059034868486e-3_f64 * t5933 * t10945 + 0.21437009059034868486e-3_f64 * t757 * t10982 + 0.12862205435420921092e-2_f64 * t5954 * t10986 - 0.12862205435420921092e-2_f64 * t5725 * t10990 + 0.12862205435420921092e-2_f64 * t2899 * t10995 - 0.42874018118069736972e-3_f64 * t299 * t10999 - 0.17149607247227894789e-2_f64 * t9308 + 0.28582678745379824648e-3_f64 * t7756 + 0.38586616306262763276e-2_f64 * t2945 * t11005;
    (t10999, t11004, t11005, t11008)
}
