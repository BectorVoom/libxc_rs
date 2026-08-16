//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1175/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1175(t13410: f64, t19727: f64, t3200: f64, t1010: f64, t10466: f64, t13382: f64, t13391: f64, t19674: f64, t19682: f64, t19686: f64, t19689: f64, t19692: f64, t19696: f64, t19700: f64, t19704: f64, t19708: f64, t19713: f64, t19717: f64, t19720: f64, t19725: f64, t4978: f64, t4981: f64, t6302: f64, t9563: f64) -> (f64, f64) {
    let t19728 = t13410 * t19727;
    let t19729 = t3200 * t19728;
    let t19732 = -0.36848765432098765431e-3_f64 * t9563 - 0.13345e0_f64 * t4981 * t4978 - 0.66725e-1_f64 * t19674 * t1010 + 0.890445125e-2_f64 * t10466 * t6302 - 0.55273148148148148147e-2_f64 * t19682 - 0.44218518518518518517e-2_f64 * t19686 + 0.66327777777777777776e-2_f64 * t19689 + 0.33163888888888888888e-2_f64 * t19692 + 0.66327777777777777776e-2_f64 * t19696 - 0.22109259259259259259e-2_f64 * t19700 - 0.22109259259259259259e-2_f64 * t19704 + 0.11054629629629629629e-2_f64 * t19708 + 0.3684876543209876543e-2_f64 * t19713 + 0.66327777777777777776e-2_f64 * t19717 - 0.66327777777777777776e-2_f64 * t19720 - 0.58958024691358024688e-2_f64 * t13382 - 0.7369753086419753086e-3_f64 * t19725 - 0.44218518518518518516e-2_f64 * t19729 + 0.22109259259259259259e-2_f64 * t13391;
    (t19729, t19732)
}
