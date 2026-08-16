//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1027/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1027(t330: f64, t4920: f64, t3393: f64, t5139: f64, t5147: f64, t8931: f64, t2943: f64, t365: f64, t11: f64, t41: f64, t85: f64, t5143: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t14915 = t4920 * t330;
    let t14926 = 0.35374814814814814814e-1_f64 * t3393 * t5139;
    let t14927 = t8931 * t5147;
    let t14940 = t365 * t2943;
    let t14954 = t11 * t41;
    let t14955 = t85 * t14954;
    let t14956 = t14955 * t5143;
    (t14915, t14926, t14927, t14940, t14955, t14956)
}
