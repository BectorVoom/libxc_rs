//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1053/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1053(t1368: f64, t16857: f64, t3971: f64, t5691: f64, t1377: f64, t5713: f64, t498: f64, t12217: f64, t3977: f64, t736: f64, t12147: f64, t5722: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t16858 = t1368 * t16857;
    let t16866 = t5691 * t3971 / 162.0_f64;
    let t16884 = t5713 * t1377;
    let t16892 = t5713 * t498;
    let t16901 = t12217 * t498;
    let t16905 = t736 * t3977;
    let t16906 = t16905 * t498;
    let t16923 = t12147 * t5722;
    (t16858, t16866, t16884, t16892, t16901, t16905, t16906, t16923)
}
