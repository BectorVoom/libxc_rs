//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1411/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1411(t10618: f64, t20957: f64, t20671: f64, t20688: f64, t26435: f64, t31207: f64, t10532: f64, t10533: f64, t34239: f64, t10520: f64, t1407: f64, t204: f64, t2476: f64, t34407: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t34949 = t20957 * t10618;
    let t34950 = 0.29792074959875355558e-1_f64 * t34949;
    let t34952 = t20688 * t20671 * t26435;
    let t34953 = 0.85206502119823888168e-1_f64 * t34952;
    let t34954 = 0.31952438294933958064e-1_f64 * t31207;
    let t34957 = 0.55213813373645879534e2_f64 * t10532 * t10533 * t34239;
    let t34959 = 0.18404604457881959845e2_f64 * t1407 * t10520;
    let t34962 = 0.92023022289409799224e1_f64 * t2476 * t204 * t34407;
    (t34950, t34953, t34954, t34957, t34959, t34962)
}
