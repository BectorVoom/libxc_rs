//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 1048/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk1048(t30232: f64, t30953: f64, t31151: f64, t31194: f64, t504: f64, t2282: f64, t27047: f64, t20922: f64, t8189: f64, t6241: f64, t8286: f64, t14294: f64) -> (f64, f64, f64, f64, f64) {
    let t31196 = t30232 + t30953 + t31151 + t31194;
    let t31197 = t31196 * t504;
    let t31199 = 3.0_f64 * t27047 * t2282;
    let t31201 = 6.0_f64 * t20922 * t8189;
    let t31203 = 3.0_f64 * t6241 * t8286;
    let t31204 = t8189 * t2282;
    let t31206 = 6.0_f64 * t14294 * t31204;
    (t31197, t31199, t31201, t31203, t31206)
}
