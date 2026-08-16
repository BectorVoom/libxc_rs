//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2553/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2553(t43859: f64, t44027: f64, t44053: f64, t50919: f64, t50948: f64, t71203: f64, t71206: f64, t71499: f64, t71501: f64, t71505: f64, t71508: f64, t71511: f64) -> f64 {
    let t71636 = 0.17938e1_f64 * t71203 + 0.53814e1_f64 * t71206 + t44027 + 0.427258125e1_f64 * t71499 - 0.230371875e0_f64 * t71501 - 0.2434271604938271605e0_f64 * t43859 - 0.49294e0_f64 * t71505 + 0.147882e1_f64 * t71508 + 0.10954222222222222222e0_f64 * t71511 - 0.26574814814814814815e0_f64 * t50919 + 0.79724444444444444446e0_f64 * t50948 + t44053;
    t71636
}
