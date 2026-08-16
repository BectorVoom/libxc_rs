//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2764/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2764(t40084: f64, t40088: f64, t40099: f64, t40103: f64, t40115: f64, t40131: f64, t50038: f64, t50039: f64, t50045: f64, t50046: f64, t50048: f64, t50051: f64, t50055: f64, t50056: f64, t50059: f64, t50063: f64, t50064: f64) -> f64 {
    let t50851 = t40084 + t40088 - t50038 + t50039 + t40099 + t40103 + t50045 - t50046 + t50048 + t50051 - t40115 + t50055 + t50056 + t50059 - t50063 + t50064 - t40131;
    t50851
}
