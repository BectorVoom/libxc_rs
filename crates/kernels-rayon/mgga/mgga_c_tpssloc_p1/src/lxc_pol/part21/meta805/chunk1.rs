//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2794/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2794(t59048: f64, t59011: f64, t59014: f64, t59015: f64, t59016: f64, t59018: f64, t59019: f64, t59020: f64, t59023: f64, t59025: f64, t59027: f64, t59029: f64, t59031: f64, t59033: f64, t59034: f64, t59035: f64, t59038: f64, t59040: f64, t59043: f64, t59046: f64) -> (f64, f64) {
    let t59049 = 0.36622894612013090108e-3_f64 * t59048;
    let t59050 = t59011 + t59014 + t59015 + t59016 + t59018 + t59019 + t59020 + t59023 + t59025 + t59027 - t59029 + t59031 + t59033 + t59034 + t59035 + t59038 + t59040 + t59043 - t59046 - t59049;
    (t59049, t59050)
}
