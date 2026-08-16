//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1190/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1190(t19905: f64, t389: f64, t19756: f64, t5181: f64, t5180: f64, t1195: f64, t6727: f64, t382: f64, t3477: f64, t6724: f64, t14721: f64, t1813: f64) -> (f64, f64, f64, f64, f64) {
    let t19906 = t19905 * t389;
    let t19908 = t5181 * t19756;
    let t19909 = t5180 * t19908;
    let t19911 = t1195 * t6727;
    let t19912 = t382 * t19911;
    let t19914 = t3477 * t6724;
    let t19916 = t14721 * t1813;
    (t19906, t19909, t19912, t19914, t19916)
}
