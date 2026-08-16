//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 1001/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk1001(t1528: f64, t4428: f64, t1524: f64, t4460: f64, t4459: f64, t512: f64, t507: f64, t1536: f64, t4437: f64, t4463: f64, t12916: f64, t12919: f64, t12922: f64, t12927: f64, t12929: f64, t12931: f64, t12933: f64, t12935: f64, t12937: f64, t12939: f64, t12943: f64, t12946: f64, t12948: f64, t12954: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t14747 = t4428 * t1528;
    let t14752 = t1524 * t4460;
    let t14756 = 1.0_f64 / t4459 / t512;
    let t14757 = t507 * t14756;
    let t14758 = t4437 * t1536;
    let t14759 = t14758 * t4463;
    let t14776 = -0.52945875e1_f64 * t12916 + 0.94674375e0_f64 * t12919 - 0.57386111111111111112e0_f64 * t12922 - 0.516475e0_f64 * t12927 - 0.68863333333333333332e0_f64 * t12929 + 0.51647499999999999999e0_f64 * t12931 + 0.34431666666666666666e0_f64 * t12933 - 0.34731666666666666667e0_f64 * t12935 + 0.20839e0_f64 * t12937 + 0.69463333333333333335e-1_f64 * t12939 - 0.46308888888888888889e-1_f64 * t12943 - 0.104195e0_f64 * t12946 - 0.103295e1_f64 * t12948 + 0.20659e1_f64 * t12954;
    (t14747, t14752, t14757, t14758, t14759, t14776)
}
