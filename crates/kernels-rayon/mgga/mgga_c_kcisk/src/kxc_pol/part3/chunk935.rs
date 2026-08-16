//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 935/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk935(t1305: f64, t4155: f64, t392: f64, t495: f64, t20: f64, t389: f64, t4001: f64, t1294: f64, t3981: f64, t1293: f64, t4000: f64, t3993: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t13851 = t4155 * t1305;
    let t13854 = 1.0_f64 / t392 / t495;
    let t13855 = t13854 * t20;
    let t13856 = t389 * t13855;
    let t13859 = t4001 * t1305;
    let t13861 = t1294 * t3981;
    let t13863 = t1293 * t4000;
    let t13866 = t3993 * t1305;
    (t13851, t13854, t13856, t13859, t13861, t13863, t13866)
}
