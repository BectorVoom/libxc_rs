//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1019/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1019(t1109: f64, t2844: f64, t1114: f64, t3255: f64, t4576: f64, t4582: f64, t4568: f64, t10386: f64, t347: f64, t1022: f64, t3201: f64, t1714: f64, t9562: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t14322 = t1109 * t2844;
    let t14326 = t1114 * t2844;
    let t14339 = 0.8760572888888888889e-3_f64 * t3255 * t4576;
    let t14341 = 0.17521145777777777778e-2_f64 * t3255 * t4582;
    let t14343 = 0.14600954814814814815e-2_f64 * t3255 * t4568;
    let t14347 = t10386 * t347;
    let t14381 = t3201 * t1022;
    let t14390 = t9562 * t1714;
    (t14322, t14326, t14339, t14341, t14343, t14347, t14381, t14390)
}
