//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 944/1333 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk944<F: Float>(t11372: F, t2889: F, t2897: F, t918: F, t11134: F, t11136: F, t11138: F, t11140: F, t11147: F, t11153: F, t11171: F, t11356: F, t11359: F, t11366: F, t11368: F, t11370: F) -> (F, F, F) {
    let t11373 = t11372 * t2889;
    let t11375 = t2897 * t918;
    let t11376 = t11375 * t2889;
    let t11378 = -F::new(0.59793333333333333333e0) * t11138 + F::new(0.11958666666666666667e1) * t11153 + F::new(0.142419375e1) * t11356 - F::new(0.76790625e-1) * t11359 - F::new(0.39862222222222222223e0) * t11134 + F::new(0.29896666666666666667e0) * t11140 + F::new(0.19931111111111111111e0) * t11136 - F::new(0.33218518518518518518e0) * t11147 - F::new(0.29896666666666666667e0) * t11171 - F::new(0.27385555555555555556e0) * t11366 + F::new(0.16431333333333333333e0) * t11368 + F::new(0.1898925e1) * t11370 - F::new(0.28483875e1) * t11373 + F::new(0.46074375e0) * t11376;
    (t11373, t11376, t11378)
}
