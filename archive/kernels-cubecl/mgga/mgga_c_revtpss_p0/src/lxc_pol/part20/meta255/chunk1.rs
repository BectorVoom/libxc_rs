//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1089/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1089<F: Float>(t11372: F, t2889: F, t2897: F, t918: F, t11134: F, t11136: F, t11138: F, t11140: F, t11147: F, t11153: F, t11171: F, t11356: F, t11359: F, t11366: F, t11368: F, t11370: F) -> (F, F, F, F) {
    let t11373 = t11372 * t2889;
    let t11375 = t2897 * t918;
    let t11376 = t11375 * t2889;
    let t11378 = -F::cast_from(0.59793333333333333333e0_f64) * t11138 + F::cast_from(0.11958666666666666667e1_f64) * t11153 + F::cast_from(0.142419375e1_f64) * t11356 - F::cast_from(0.76790625e-1_f64) * t11359 - F::cast_from(0.39862222222222222223e0_f64) * t11134 + F::cast_from(0.29896666666666666667e0_f64) * t11140 + F::cast_from(0.19931111111111111111e0_f64) * t11136 - F::cast_from(0.33218518518518518518e0_f64) * t11147 - F::cast_from(0.29896666666666666667e0_f64) * t11171 - F::cast_from(0.27385555555555555556e0_f64) * t11366 + F::cast_from(0.16431333333333333333e0_f64) * t11368 + F::cast_from(0.1898925e1_f64) * t11370 - F::cast_from(0.28483875e1_f64) * t11373 + F::cast_from(0.46074375e0_f64) * t11376;
    (t11373, t11375, t11376, t11378)
}
