//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 856/1505 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk856<F: Float>(t2299: F, t5819: F, t5825: F, t633: F, t2306: F, t637: F, t77: F, t1471: F, t1487: F, t1494: F, t5820: F, t5827: F, t5830: F, t5855: F, t71: F, t85: F) -> (F, F) {
    let t5860 = t2299 * t5819;
    let t5862 = t633 * t5825;
    let t5864 = t2306 * t5819;
    let t5866 = t637 * t5825;
    let t5868 = F::new(28.0) / F::new(9.0) * t5860 - F::new(4.0) / F::new(3.0) * t5862 + F::new(28.0) / F::new(9.0) * t5864 + F::new(4.0) / F::new(3.0) * t5866;
    let t5869 = t77 * t5868;
    let t5872 = -t5820 * t85 / F::new(12.0) - t5827 * t85 / F::new(12.0) - t5830 * t85 / F::new(6.0) - t1471 * t1494 / F::new(6.0) + t5855 * t85 / F::new(24.0) + t1487 * t1494 / F::new(12.0) + t71 * t5869 / F::new(24.0);
    (t5869, t5872)
}
