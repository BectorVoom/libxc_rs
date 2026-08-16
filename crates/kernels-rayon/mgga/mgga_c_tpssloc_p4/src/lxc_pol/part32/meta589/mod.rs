//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta589 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1977;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta589(t2239: f64, t3951: f64, t193: f64, t776: f64, t111: f64, t5363: f64, t6470: f64, t19297: f64, t604: f64, t5385: f64, t19449: f64, t19644: f64, t225: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t46104, t46341, t55353, t55388, t55880, t55921, t55943, t56422) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1977(t2239, t3951, t193, t776, t111, t5363, t6470, t19297, t604, t5385, t19449, t19644, t225);
    (t46104, t46341, t55353, t55388, t55880, t55921, t55943, t56422)
}
