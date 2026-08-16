//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 737/1304 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk737(t1170: f64, t2148: f64, t2121: f64, t225: f64, t7284: f64) -> (f64, f64, f64) {
    let t7359 = t1170 * t2148;
    let t7361 = 0.27415567780803773942e-2_f64 * t2121 * t7359;
    let t7362 = t7284 * t225;
    (t7359, t7361, t7362)
}
