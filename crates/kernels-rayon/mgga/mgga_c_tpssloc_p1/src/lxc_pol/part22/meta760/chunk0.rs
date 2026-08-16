//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2560/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2560(t4745: f64, t64257: f64, t4786: f64, t63755: f64, t14838: f64, t18255: f64, t14850: f64, t18259: f64, t11303: f64, t1136: f64, t11361: f64, t11420: f64, t15146: f64, t15207: f64, t1683: f64, t1694: f64, t18615: f64, t18623: f64, t18631: f64, t18634: f64, t18893: f64, t21839: f64, t21842: f64, t21952: f64, t3332: f64, t3357: f64, t3401: f64, t4819: f64, t4820: f64, t4857: f64, t51376: f64, t6037: f64, t6052: f64, t63533: f64) -> (f64, f64, f64, f64, f64) {
    let t71784 = 6.0_f64 * t64257 * t4745;
    let t71786 = 0.48245938496077605201e2_f64 * t63755 * t4786;
    let t71788 = 6.0_f64 * t14838 * t18255;
    let t71790 = 0.48245938496077605201e2_f64 * t14850 * t18259;
    let t71791 = 0.30762056574649219972e4_f64 * t51376 * t18623 + 0.51947577317044391277e2_f64 * t11361 * t21839 + 0.51947577317044391277e2_f64 * t3401 * t63533 * t1694 + 0.51947577317044391277e2_f64 * t3401 * t18615 * t4857 + 18.0_f64 * t15146 * t18631 - 12.0_f64 * t15207 * t18634 - 24.0_f64 * t11420 * t21952 * t1136 + 18.0_f64 * t3357 * t6037 * t4819 - 6.0_f64 * t11303 * t21842 - 6.0_f64 * t3332 * t4820 * t6052 - 6.0_f64 * t3332 * t1683 * t18893 + t71784 - t71786 + t71788 - t71790;
    (t71784, t71786, t71788, t71790, t71791)
}
