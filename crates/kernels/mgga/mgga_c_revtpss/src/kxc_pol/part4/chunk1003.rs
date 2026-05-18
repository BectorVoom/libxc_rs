//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1003/1428 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk1003<F: Float>(t123: F, t752: F, t2630: F, t2629: F, t9866: F, t9575: F, t9572: F, t177: F, t2390: F, t762: F, t760: F, t9419: F) -> (F, F, F, F, F, F) {
    let t10578 = t752 * t123;
    let t10579 = t10578 * t2630;
    let t10582 = F::new(0.48159733137676571078e0) * t2629 * t9866;
    let t10584 = F::new(0.21687162600603479684e-1) * t2629 * t9575;
    let t10586 = F::new(0.32530743900905219526e-1) * t2629 * t9572;
    let t10587 = t2390 * t177;
    let t10588 = t10587 * t762;
    let t10592 = F::new(0.10389515463408878255e3) * t760 * t9419;
    (t10579, t10582, t10584, t10586, t10588, t10592)
}
