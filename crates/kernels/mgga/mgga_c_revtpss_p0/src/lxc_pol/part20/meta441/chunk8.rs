//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1683/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1683<F: Float>(t45015: F, t45021: F, t45023: F, t45026: F, t45029: F, t45033: F, t45037: F, t45040: F, t45043: F, t45045: F, t45048: F, t45050: F, t45052: F) -> F {
    let t45903 = -t45015 + t45021 + t45023 - t45026 - t45029 + t45033 + t45037 + t45040 + t45043 + t45045 + t45048 + t45050 - t45052;
    t45903
}
