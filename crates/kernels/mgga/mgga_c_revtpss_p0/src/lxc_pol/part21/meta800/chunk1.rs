//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2902/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2902<F: Float>(t51890: F, t51892: F, t51894: F, t51896: F, t51899: F, t51902: F, t51907: F, t51909: F, t51911: F, t51913: F, t51915: F, t51917: F) -> F {
    let t52677 = -F::new(0.28483875e1) * t51890 - F::new(0.9494625e0) * t51892 + F::new(0.46074375e0) * t51894 + F::new(0.15358125e0) * t51896 - F::cast_from(0.3560484375e1_f64) * t51899 + F::cast_from(0.1151859375e0_f64) * t51902 - F::cast_from(0.82156666666666666668e-1_f64) * t51907 - F::cast_from(0.65725333333333333332e0_f64) * t51909 + F::cast_from(0.10954222222222222222e0_f64) * t51911 + F::cast_from(0.5477111111111111111e0_f64) * t51913 - F::cast_from(0.91285185185185185185e-1_f64) * t51915 - F::cast_from(0.32862666666666666666e0_f64) * t51917;
    t52677
}
