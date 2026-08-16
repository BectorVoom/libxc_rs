//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2899/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2899<F: Float>(t41406: F, t52045: F, t52047: F, t52049: F, t52051: F, t52054: F, t52057: F, t52060: F, t52063: F, t52065: F, t52068: F, t52116: F) -> F {
    let t52615 = -F::cast_from(0.20658999999999999999e1_f64) * t52045 + F::cast_from(0.68863333333333333333e0_f64) * t52047 + F::cast_from(0.34431666666666666666e0_f64) * t52049 + F::cast_from(0.57386111111111111111e0_f64) * t52051 - F::cast_from(0.103295e1_f64) * t52054 - F::cast_from(0.103295e1_f64) * t52057 - F::cast_from(0.17215833333333333333e1_f64) * t52060 - F::cast_from(0.929655e1_f64) * t52063 + F::cast_from(0.20839e0_f64) * t52065 - F::cast_from(0.104195e0_f64) * t52068 + F::cast_from(0.69463333333333333332e-1_f64) * t41406 + F::cast_from(0.6311625e0_f64) * t52116;
    t52615
}
