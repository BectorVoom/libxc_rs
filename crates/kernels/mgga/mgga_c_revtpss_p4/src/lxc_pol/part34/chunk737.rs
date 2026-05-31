//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 737/1341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk737<F: Float>(t5: F, t1923: F, t1928: F, t6958: F, t7702: F, t7706: F, t7709: F, t7716: F, t7720: F, t117: F, t1937: F, t4248: F, t1518: F, t94: F) -> (F, F, F, F) {
    let t7 = piecewise3::<F>(F::cast_from(0.0_f64) < t5, t5, -t5);
    let t8 = -t7 <= -F::cast_from(0.999999999999e0_f64);
    let t7724 = piecewise3::<F>(t8, F::cast_from(0.0_f64), -t7702 * t1928 / F::cast_from(6.0_f64) + F::cast_from(5.0_f64) / F::cast_from(6.0_f64) * t6958 * t7706 + t7709 * t1928 / F::cast_from(3.0_f64) - t1923 * t7716 / F::cast_from(6.0_f64) - t1923 * t7720 / F::cast_from(6.0_f64));
    let t7725 = t7724 * t117;
    let t7731 = F::cast_from(2.0_f64) * t4248 * t1937;
    let t7732 = t94 * t1518;
    (t7724, t7725, t7731, t7732)
}
