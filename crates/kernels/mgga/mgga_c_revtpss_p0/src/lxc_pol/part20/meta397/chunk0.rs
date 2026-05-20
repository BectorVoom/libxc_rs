//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1465/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1465<F: Float>(t41316: F, t41323: F, t41353: F, t41356: F, t41359: F, t41396: F, t41402: F, t41404: F, t41406: F, t41409: F, t41412: F, t41414: F, t41417: F, t41419: F) -> F {
    let t41717 = -F::new(0.185931e2) * t41316 + F::new(0.123954e2) * t41323 - F::cast_from(0.34431666666666666667e1_f64) * t41353 + F::new(0.41318e1) * t41356 - F::cast_from(0.13772666666666666667e1_f64) * t41359 + F::new(0.3529725e1) * t41396 - F::cast_from(0.6618234375e1_f64) * t41402 - F::new(0.52945875e1) * t41404 + F::cast_from(0.27785333333333333333e0_f64) * t41406 - F::new(0.375102e1) * t41409 + F::cast_from(0.158837625e2_f64) * t41412 - F::new(0.705945e1) * t41414 - F::new(0.94674375e0) * t41417 + F::new(0.1262325e1) * t41419;
    t41717
}
