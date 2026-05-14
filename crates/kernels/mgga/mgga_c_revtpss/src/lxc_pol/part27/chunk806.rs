//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 806/1170 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk806<F: Float>(t123: F, t752: F, t2630: F, t2629: F, t9866: F, t9575: F, t9572: F, t177: F, t2390: F, t762: F, t10575: F, t10577: F, t9514: F, t9517: F, t9521: F, t9524: F) -> (F, F, F, F, F, F) {
    let t10578 = t752 * t123;
    let t10579 = t10578 * t2630;
    let t10580 = 0.32530743900905219526e-1 * t10579;
    let t10582 = 0.48159733137676571078e0 * t2629 * t9866;
    let t10584 = 0.21687162600603479684e-1 * t2629 * t9575;
    let t10586 = 0.32530743900905219526e-1 * t2629 * t9572;
    let t10587 = t2390 * t177;
    let t10588 = t10587 * t762;
    let t10589 = 0.17544670867903938621e1 * t10588;
    let t10590 = -t10575 + t9514 - t9517 - t9521 + t10577 + t10580 + t10582 - t10584 - t10586 - t9524 - t10589;
    (t10580, t10582, t10584, t10586, t10589, t10590)
}
