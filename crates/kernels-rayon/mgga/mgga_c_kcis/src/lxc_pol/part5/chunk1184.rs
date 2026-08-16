//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1184/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1184(t1203: f64, t6735: f64, t3330: f64, t1808: f64, t3436: f64, t5182: f64, t19630: f64, t3438: f64, t3437: f64, t1196: f64, t6709: f64, t10787: f64, t6693: f64) -> (f64, f64, f64, f64, f64) {
    let t19843 = t6735 * t1203;
    let t19845 = 2.0_f64 * t3330 * t19843;
    let t19846 = t1808 * t3436;
    let t19847 = t19846 * t5182;
    let t19849 = t3438 * t19630;
    let t19850 = t3437 * t19849;
    let t19852 = t6709 * t1196;
    let t19854 = t10787 * t6693;
    (t19845, t19847, t19850, t19852, t19854)
}
