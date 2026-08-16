//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1342/1464 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1342(t10520: f64, t1407: f64, t204: f64, t2476: f64, t34407: f64, t10615: f64, t30848: f64, t34371: f64, t6710: f64, t6711: f64, t34321: f64, t6717: f64, t6914: f64) -> (f64, f64, f64, f64, f64) {
    let t34959 = 0.18404604457881959845e2_f64 * t1407 * t10520;
    let t34962 = 0.92023022289409799224e1_f64 * t2476 * t204 * t34407;
    let t34964 = 0.50050685932590597338e1_f64 * t10615 * t30848;
    let t34967 = 0.23005755572352449806e2_f64 * t6710 * t6711 * t34371;
    let t34970 = 0.12423108009070322895e3_f64 * t6914 * t6717 * t34321;
    (t34959, t34962, t34964, t34967, t34970)
}
