//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1340/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1340(t1267: f64, t26996: f64, t5329: f64, t5336: f64, t1856: f64, t3620: f64, t92735: f64, t15573: f64, t28136: f64, t27077: f64, t28137: f64, t7772: f64, t8091: f64, t92587: f64, t92600: f64, t92604: f64, t92607: f64, t92732: f64, t95532: f64, t95542: f64, t95545: f64) -> (f64, f64, f64, f64) {
    let t96714 = t5329 * t26996 * t5336 * t1267;
    let t96720 = t5329 * t92735 * t1856 * t3620;
    let t96727 = t15573 * t28136;
    let t96728 = t27077 * t96727;
    let t96732 = 0.46429444444444444444e-2_f64 * t95532 + 0.38691203703703703703e-3_f64 * t95542 - 0.51485339506172839507e-4_f64 * t92587 - 0.92754700520833333334e-4_f64 * t7772 * t96714 + 0.51485339506172839506e-4_f64 * t92600 + 0.51015085286458333333e-3_f64 * t7772 * t96720 - 0.7722800925925925926e-4_f64 * t92607 + 0.49512459138020833334e-4_f64 * t92732 * t28137 + 0.77382407407407407406e-3_f64 * t95545 - 0.61890573922526041667e-5_f64 * t96728 + 0.61782407407407407408e-3_f64 * t92604 * t8091;
    (t96714, t96720, t96727, t96732)
}
