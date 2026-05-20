//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1151/1333 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1151<F: Float>(t2247: F, t26754: F, t2282: F, t55: F, t2251: F, t2258: F, t25137: F, t7571: F, t72: F, t1927: F, t6977: F, t7575: F) -> (F, F, F, F, F, F) {
    let t26755 = t2247 * t26754;
    let t26776 = t55 * t2282;
    let t26781 = F::new(5.0) / F::new(18.0) * t26776 * t2251 - F::new(5.0) / F::new(6.0) * t7571 * t2258 - t25137;
    let t26782 = t26781 * t72;
    let t26783 = t26782 * t1927;
    let t26786 = t7575 * t6977;
    (t26755, t26776, t26781, t26782, t26783, t26786)
}
