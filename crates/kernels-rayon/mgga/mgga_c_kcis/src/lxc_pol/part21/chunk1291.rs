//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1291/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1291(t7703: f64, t95684: f64, t26685: f64, t26692: f64, t27950: f64, t92730: f64, t93403: f64, t93406: f64, t93409: f64, t93437: f64, t95756: f64, t95759: f64, t95764: f64, t95769: f64, t95775: f64) -> f64 {
    let t95779 = 0.46336805555555555556e-3_f64 * t7703 * t95684;
    let t95780 = 0.44218518518518518517e-2_f64 * t95756 - 0.66327777777777777776e-2_f64 * t95759 + 0.16475308641975308642e-2_f64 * t26692 * t27950 - 0.20594135802469135802e-3_f64 * t95764 - 0.15445601851851851852e-3_f64 * t93403 - 0.7722800925925925926e-4_f64 * t93406 - 0.10297067901234567901e-3_f64 * t93409 - 0.556528203125e-3_f64 * t26685 * t95769 - 0.46336805555555555556e-3_f64 * t93437 - 0.92673611111111111113e-3_f64 * t95775 - 0.73697530864197530861e-3_f64 * t92730 - t95779;
    t95780
}
