//! MGGA_C_REVTPSS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 937/1286 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part5_v3rho3_2_chunk937<F: Float>(t755: F, t9586: F, t2619: F, t2622: F, t2629: F, t9863: F, t123: F, t752: F, t2630: F, t9866: F, t9575: F, t9572: F, t760: F, t9419: F, t2516: F, t2523: F) -> (F, F, F, F, F, F, F, F, F) {
    let t10568 = 0.56968947174242584612e-3 * t755 * t9586;
    let t10569 = t2622 * t2619;
    let t10577 = 0.16265371950452609763e-1 * t2629 * t9863;
    let t10578 = t752 * t123;
    let t10579 = t10578 * t2630;
    let t10582 = 0.48159733137676571078e0 * t2629 * t9866;
    let t10584 = 0.21687162600603479684e-1 * t2629 * t9575;
    let t10586 = 0.32530743900905219526e-1 * t2629 * t9572;
    let t10592 = 0.10389515463408878255e3 * t760 * t9419;
    let t10593 = t2523 * t2516;
    (t10568, t10569, t10577, t10579, t10582, t10584, t10586, t10592, t10593)
}
