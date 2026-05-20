//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 1225/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1225<F: Float>(t38: F, t5854: F, t2299: F, t5819: F, t5825: F, t633: F, t2306: F, t637: F, t77: F) -> (F, F) {
    let t5855 = t38 * t5854;
    let t5860 = t2299 * t5819;
    let t5862 = t633 * t5825;
    let t5864 = t2306 * t5819;
    let t5866 = t637 * t5825;
    let t5868 = F::new(28.0) / F::new(9.0) * t5860 - F::new(4.0) / F::new(3.0) * t5862 + F::new(28.0) / F::new(9.0) * t5864 + F::new(4.0) / F::new(3.0) * t5866;
    let t5869 = t77 * t5868;
    (t5855, t5869)
}
