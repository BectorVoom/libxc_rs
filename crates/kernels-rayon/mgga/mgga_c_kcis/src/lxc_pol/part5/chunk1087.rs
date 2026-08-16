//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1087/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1087(t3255: f64, t6586: f64, t10271: f64, t10414: f64, t1102: f64, t14115: f64, t14321: f64, t18536: f64, t18539: f64, t18543: f64, t18548: f64, t18552: f64, t18556: f64, t18559: f64, t18563: f64, t18567: f64, t18571: f64, t18575: f64, t18579: f64, t18582: f64, t18584: f64, t18586: f64, t18588: f64, t18590: f64, t4587: f64) -> f64 {
    let t18592 = t3255 * t6586;
    let t18594 = 0.32852148333333333333e-2_f64 * t14321 * t18536 - 0.19711289e-2_f64 * t10414 * t18539 + t10271 - 0.295669335e-2_f64 * t1102 * t18543 + 0.295669335e-2_f64 * t1102 * t18548 - 0.59133867e-2_f64 * t1102 * t18552 + 0.39422578e-2_f64 * t1102 * t18556 - 0.19711289e-2_f64 * t18559 - 0.2920190962962962963e-3_f64 * t14115 - 0.19711289e-2_f64 * t1102 * t18563 + 0.13140859333333333333e-2_f64 * t1102 * t18567 + 0.39422577999999999999e-2_f64 * t1102 * t18571 - 0.52563437333333333332e-2_f64 * t4587 * t18575 + 0.98556445e-3_f64 * t1102 * t18579 + 0.13140859333333333333e-2_f64 * t18582 - 0.87605728888888888887e-3_f64 * t18584 - 0.65704296666666666667e-3_f64 * t18586 + 0.73004774074074074073e-3_f64 * t18588 - 0.87605728888888888887e-3_f64 * t18590 + 0.43802864444444444445e-3_f64 * t18592;
    t18594
}
