//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 841/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk841(t1368: f64, t16852: f64, t12133: f64, t1933: f64, t3971: f64, t5691: f64, t1377: f64, t5713: f64, t498: f64, t12217: f64, t3977: f64, t736: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t16854 = t1368 * t16852 / 72.0_f64;
    let t16857 = t12133 * t1933;
    let t16858 = t1368 * t16857;
    let t16866 = t5691 * t3971 / 162.0_f64;
    let t16884 = t5713 * t1377;
    let t16892 = t5713 * t498;
    let t16901 = t12217 * t498;
    let t16905 = t736 * t3977;
    (t16854, t16858, t16866, t16884, t16892, t16901, t16905)
}
