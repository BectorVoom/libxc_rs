//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2906/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2906<F: Float>(t41406: F, t52045: F, t52047: F, t52049: F, t52051: F, t52054: F, t52057: F, t52060: F, t52063: F, t52065: F, t52068: F, t52116: F) -> F {
    let t52743 = -F::cast_from(0.11958666666666666667e1_f64) * t52045 + F::cast_from(0.39862222222222222222e0_f64) * t52047 + F::cast_from(0.19931111111111111112e0_f64) * t52049 + F::cast_from(0.33218518518518518519e0_f64) * t52051 - F::cast_from(0.59793333333333333333e0_f64) * t52054 - F::cast_from(0.59793333333333333333e0_f64) * t52057 - F::cast_from(0.99655555555555555555e0_f64) * t52060 - F::cast_from(0.53814000000000000001e1_f64) * t52063 + F::cast_from(0.16431333333333333333e0_f64) * t52065 - F::cast_from(0.82156666666666666667e-1_f64) * t52068 + F::cast_from(0.54771111111111111111e-1_f64) * t41406 + F::new(0.3071625e0) * t52116;
    t52743
}
