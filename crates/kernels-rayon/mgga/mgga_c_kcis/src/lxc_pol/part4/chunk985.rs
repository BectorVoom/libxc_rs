//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 985/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk985(t977: f64, t278: f64, t2835: f64, t975: f64, t119: f64, t251: f64, t85: f64, t361: f64, t1127: f64, t3245: f64, t2822: f64, t2852: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t10461 = t977 * t977;
    let t10462 = 1.0_f64 / t10461;
    let t10463 = t278 * t10462;
    let t10466 = t975 * t2835;
    let t10470 = t85 * t119 * t251;
    let t10471 = t10470 * t361;
    let t10472 = 0.73697530864197530862e-3_f64 * t10471;
    let t10473 = t3245 * t1127;
    let t10477 = t2822 * t2852;
    (t10463, t10466, t10470, t10471, t10472, t10473, t10477)
}
