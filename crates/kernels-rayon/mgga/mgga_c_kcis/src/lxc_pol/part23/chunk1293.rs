//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1293/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1293(t5440: f64, t99198: f64, t99199: f64, t1307: f64, t28814: f64, t95024: f64, t3978: f64, t7969: f64, t5426: f64, t1370: f64, t27636: f64, t27606: f64, t6140: f64) -> (f64, f64, f64, f64, f64) {
    let t99201 = t99198 * t5440 * t99199;
    let t99205 = t95024 * t28814 * t1307;
    let t99208 = t3978 * t7969;
    let t99210 = t99208 * t5426 * t99199;
    let t99213 = t1370 * t27636;
    let t99219 = t27606 * t6140;
    (t99201, t99205, t99210, t99213, t99219)
}
