//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 992/1422 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk992<F: Float>(t10565: F, t158: F, t755: F, t9586: F, t2619: F, t2622: F, t2629: F, t9863: F, t123: F, t752: F, t2630: F, t9866: F) -> (F, F, F, F, F, F) {
    let t10566 = t158 * t10565;
    let t10568 = F::new(0.56968947174242584612e-3) * t755 * t9586;
    let t10569 = t2622 * t2619;
    let t10577 = F::new(0.16265371950452609763e-1) * t2629 * t9863;
    let t10578 = t752 * t123;
    let t10579 = t10578 * t2630;
    let t10582 = F::new(0.48159733137676571078e0) * t2629 * t9866;
    (t10566, t10568, t10569, t10577, t10579, t10582)
}
