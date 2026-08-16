//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2236/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2236(t16924: f64, t23146: f64, t16914: f64, t16903: f64, t5593: f64, t81749: f64, t16845: f64, t25084: f64, t16893: f64, t17017: f64, t16841: f64, t87368: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t98612 = t23146 * t16924;
    let t98614 = t23146 * t16914;
    let t98616 = t23146 * t16903;
    let t98618 = t81749 * t5593;
    let t98620 = t25084 * t16845;
    let t98622 = t25084 * t16893;
    let t98624 = t23146 * t17017;
    let t98626 = t87368 * t16841;
    (t98612, t98614, t98616, t98618, t98620, t98622, t98624, t98626)
}
