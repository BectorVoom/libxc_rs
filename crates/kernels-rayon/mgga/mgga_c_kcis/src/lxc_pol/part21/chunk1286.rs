//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1286/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1286(t27779: f64, t93435: f64, t26685: f64, t13354: f64, t26760: f64, t4994: f64, t27772: f64, t27778: f64, t93427: f64, t14443: f64, t27825: f64, t7703: f64) -> (f64, f64, f64, f64, f64) {
    let t95684 = t93435 * t27779;
    let t95686 = 0.61836467013888888889e-4_f64 * t26685 * t95684;
    let t95688 = t4994 * t26760 * t13354;
    let t95691 = t27772 * t27778 * t93427;
    let t95696 = 0.30891203703703703704e-3_f64 * t7703 * t14443 * t27825;
    (t95684, t95686, t95688, t95691, t95696)
}
