//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2884/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2884<F: Float>(t23421: F, t892: F, t18865: F, t18871: F, t18875: F, t2403: F, t77029: F, t77032: F, t77036: F, t77038: F, t77039: F, t77040: F, t77041: F, t77045: F, t775: F) -> F {
    let t77460 = t23421 * t892;
    let t77467 = -F::cast_from(9.0_f64) * t18865 * t18875 * t2403 + F::cast_from(18.0_f64) * t18871 * t18875 * t2403 + F::cast_from(3.0_f64) * t2403 * t77460 * t775 + t77029 + t77032 + t77036 + t77038 + t77039 + t77040 + t77041 + t77045;
    t77467
}
