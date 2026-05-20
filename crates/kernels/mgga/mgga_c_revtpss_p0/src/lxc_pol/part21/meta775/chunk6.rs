//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2764/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2764<F: Float>(t40084: F, t40088: F, t40099: F, t40103: F, t40115: F, t40131: F, t50038: F, t50039: F, t50045: F, t50046: F, t50048: F, t50051: F, t50055: F, t50056: F, t50059: F, t50063: F, t50064: F) -> F {
    let t50851 = t40084 + t40088 - t50038 + t50039 + t40099 + t40103 + t50045 - t50046 + t50048 + t50051 - t40115 + t50055 + t50056 + t50059 - t50063 + t50064 - t40131;
    t50851
}
