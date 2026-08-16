//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 1013/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk1013(t26: f64, t30631: f64, t1186: f64, t30298: f64, t12999: f64, t13000: f64, t19543: f64, t30592: f64, t30595: f64, t30599: f64, t30603: f64, t30613: f64, t30617: f64, t30623: f64, t30626: f64, t30629: f64) -> (f64, f64, f64) {
    let t30632 = t26 * t30631;
    let t30634 = t1186 * t30298;
    let t30635 = t26 * t30634;
    let t30637 = 0.46074375e0_f64 * t30613 - t12999 - t13000 - 0.27385555555555555556e0_f64 * t19543 + 0.142419375e1_f64 * t30617 + 0.11958666666666666667e1_f64 * t30595 - 0.17938e1_f64 * t30599 - 0.33218518518518518518e0_f64 * t30592 - 0.29896666666666666667e0_f64 * t30603 - 0.76790625e-1_f64 * t30623 - 0.36514074074074074075e-1_f64 * t30626 - 0.82156666666666666667e-1_f64 * t30629 + 0.16431333333333333333e0_f64 * t30632 - 0.49293999999999999999e0_f64 * t30635;
    (t30632, t30635, t30637)
}
