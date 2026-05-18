//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 824/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk824<F: Float>(t2390: F, t72: F, t757: F, t2629: F, t9863: F, t123: F, t752: F, t2630: F, t9866: F, t9575: F, t9572: F, t177: F) -> (F, F, F, F, F, F, F) {
    let t10573 = t2390 * t72;
    let t10574 = t10573 * t757;
    let t10575 = F::new(0.54934341918019635162e-3) * t10574;
    let t10577 = F::new(0.16265371950452609763e-1) * t2629 * t9863;
    let t10578 = t752 * t123;
    let t10579 = t10578 * t2630;
    let t10580 = F::new(0.32530743900905219526e-1) * t10579;
    let t10582 = F::new(0.48159733137676571078e0) * t2629 * t9866;
    let t10584 = F::new(0.21687162600603479684e-1) * t2629 * t9575;
    let t10586 = F::new(0.32530743900905219526e-1) * t2629 * t9572;
    let t10587 = t2390 * t177;
    (t10575, t10577, t10580, t10582, t10584, t10586, t10587)
}
