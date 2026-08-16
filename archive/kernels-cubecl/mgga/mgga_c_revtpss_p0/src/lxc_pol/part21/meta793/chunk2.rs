//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2869/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2869<F: Float>(t52090: F, t52114: F, t923: F, t41406: F, t52045: F, t52047: F, t52049: F, t52051: F, t52054: F, t52057: F, t52060: F, t52063: F, t52065: F, t52068: F) -> (F, F, F) {
    let t52115 = t52090 + t52114;
    let t52116 = t923 * t52115;
    let t52118 = -F::cast_from(0.12077e1_f64) * t52045 + F::cast_from(0.40256666666666666667e0_f64) * t52047 + F::cast_from(0.20128333333333333333e0_f64) * t52049 + F::cast_from(0.33547222222222222222e0_f64) * t52051 - F::cast_from(0.60384999999999999999e0_f64) * t52054 - F::cast_from(0.60384999999999999999e0_f64) * t52057 - F::cast_from(0.10064166666666666666e1_f64) * t52060 - F::cast_from(0.543465e1_f64) * t52063 + F::cast_from(0.16557e0_f64) * t52065 - F::cast_from(0.82785e-1_f64) * t52068 + F::cast_from(0.55190000000000000001e-1_f64) * t41406 + F::cast_from(0.16504875e0_f64) * t52116;
    (t52115, t52116, t52118)
}
