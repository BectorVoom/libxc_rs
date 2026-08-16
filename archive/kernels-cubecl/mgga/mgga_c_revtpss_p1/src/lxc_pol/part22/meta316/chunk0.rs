//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 1759/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1759<F: Float>(t10565: F, t158: F, t755: F, t9586: F, t2619: F, t2622: F, t2390: F, t72: F, t757: F, t2629: F, t9863: F, t123: F, t752: F) -> (F, F, F, F, F, F, F) {
    let t10566 = t158 * t10565;
    let t10568 = F::cast_from(0.56968947174242584612e-3_f64) * t755 * t9586;
    let t10569 = t2622 * t2619;
    let t10573 = t2390 * t72;
    let t10574 = t10573 * t757;
    let t10577 = F::cast_from(0.16265371950452609763e-1_f64) * t2629 * t9863;
    let t10578 = t752 * t123;
    (t10566, t10568, t10569, t10573, t10574, t10577, t10578)
}
