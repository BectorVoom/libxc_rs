//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2345/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2345(t5456: f64, t7263: f64, t2109: f64, t96461: f64, t96469: f64, t96425: f64, t22549: f64, t24514: f64, t24517: f64, t26016: f64, t27298: f64, t83717: f64, t85501: f64, t90098: f64, t90101: f64, t90104: f64, t96135: f64, t96138: f64, t96418: f64, t96422: f64, t96466: f64, t96473: f64) -> (f64, f64) {
    let t104729 = t7263 * t5456;
    let t104735 = t2109 * t96461;
    let t104740 = t2109 * t96469;
    let t104749 = t2109 * t96425;
    let t104758 = 35.0_f64 * t85501 * t96418 - 10.0_f64 * t24514 * t96422 - 10.0_f64 / 3.0_f64 * t22549 * t104735 - 5.0_f64 * t24514 * t96466 - 5.0_f64 / 3.0_f64 * t22549 * t104740 - 5.0_f64 / 3.0_f64 * t96473 * t24517 - 10.0_f64 / 3.0_f64 * t26016 * t96135 - 10.0_f64 / 3.0_f64 * t26016 * t96138 + 10.0_f64 * t83717 * t104749 - 10.0_f64 / 3.0_f64 * t90098 * t27298 - 10.0_f64 / 3.0_f64 * t90101 * t27298 - 10.0_f64 / 3.0_f64 * t90104 * t27298;
    (t104729, t104758)
}
