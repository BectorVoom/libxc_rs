//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1446/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1446<F: Float>(t11354: F, t2881: F, t2889: F, t11315: F, t11372: F, t11358: F, t11375: F, t41316: F, t41323: F, t41353: F, t41356: F, t41359: F, t41396: F, t41402: F, t41404: F, t41406: F, t41409: F) -> (F, F, F, F, F) {
    let t41412 = t11354 * t2881 * t2889;
    let t41414 = t11372 * t11315;
    let t41417 = t11358 * t2881 * t2889;
    let t41419 = t11375 * t11315;
    let t41421 = -F::new(0.108693e2) * t41316 + F::new(0.72462e1) * t41323 - F::cast_from(0.20128333333333333334e1_f64) * t41353 + F::new(0.24154e1) * t41356 - F::cast_from(0.80513333333333333332e0_f64) * t41359 + F::new(0.258925e1) * t41396 - F::cast_from(0.485484375e1_f64) * t41402 - F::new(0.3883875e1) * t41404 + F::new(0.22076e0) * t41406 - F::new(0.298026e1) * t41409 + F::new(0.11651625e2) * t41412 - F::new(0.51785e1) * t41414 - F::cast_from(0.247573125e0_f64) * t41417 + F::new(0.3300975e0) * t41419;
    (t41412, t41414, t41417, t41419, t41421)
}
