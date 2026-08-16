//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 861/1203 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk861<F: Float>(t114: F, t25822: F, t25824: F, t28034: F, t28037: F, t28039: F) -> F {
    let t115 = F::cast_from(1.0_f64) < t114;
    let t28042 = piecewise3::<F>(t115, F::cast_from(0.0_f64), t25822 + t25824 / F::cast_from(3.0_f64) + t28034 / F::cast_from(3.0_f64) + t28037 / F::cast_from(4.0_f64) - t28039 / F::cast_from(8.0_f64));
    t28042
}
