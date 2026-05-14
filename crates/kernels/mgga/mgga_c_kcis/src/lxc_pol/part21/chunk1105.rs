//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1105/1221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1105<F: Float>(t7690: F, t93661: F, t93637: F, t26807: F, t7703: F, t9938: F, t26714: F, t7696: F, t26717: F, t2173: F, t10466: F, t3489: F, t26739: F, t1250: F, t33827: F, t15573: F, t26792: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t93662 = t7690 * t93661;
    let t93664 = t7690 * t93637;
    let t93686 = t7703 * t9938 * t26807;
    let t93690 = t7696 * t26714;
    let t93694 = t7696 * t26717;
    let t93704 = t2173 * t93637;
    let t93709 = t10466 * t3489;
    let t93714 = t26739 * t26717;
    let t93718 = t33827 * t1250;
    let t93728 = t2173 * t15573 * t26792;
    (t93662, t93664, t93686, t93690, t93694, t93704, t93709, t93714, t93718, t93728)
}
