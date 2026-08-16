//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1209/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1209(t20191: f64, t382: f64, t1195: f64, t6723: f64, t1187: f64, t19593: f64, t5181: f64, t3437: f64, t19735: f64, t3438: f64, t1809: f64, t5086: f64) -> (f64, f64, f64, f64, f64) {
    let t20192 = t382 * t20191;
    let t20194 = t1195 * t6723;
    let t20195 = t1187 * t20194;
    let t20197 = t5181 * t19593;
    let t20198 = t3437 * t20197;
    let t20200 = t3438 * t19735;
    let t20201 = t3437 * t20200;
    let t20203 = t1809 * t5086;
    (t20192, t20195, t20198, t20201, t20203)
}
