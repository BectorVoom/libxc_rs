//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta137 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk765;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta137(t3584: f64, t61: f64, t1236: f64, t225: f64, t1239: f64, t496: f64, t68: f64, t1243: f64, t3534: f64, t3032: f64, t3502: f64, t3499: f64) -> (f64, f64, f64, f64, f64, f64) {
        let (t3585, t3593, t3598, t3604, t3609, t3610) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk765(t3584, t61, t1236, t225, t1239, t496, t68, t1243, t3534, t3032, t3502, t3499);
    (t3585, t3593, t3598, t3604, t3609, t3610)
}
