//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1249/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1249<F: Float>(t46978: F, t7692: F, t7690: F, t93637: F, t26807: F, t7703: F, t9938: F, t26714: F, t7696: F, t26717: F, t2173: F, t10466: F, t3489: F) -> (F, F, F, F, F, F, F, F) {
    let t93661 = t46978 * t7692;
    let t93662 = t7690 * t93661;
    let t93664 = t7690 * t93637;
    let t93686 = t7703 * t9938 * t26807;
    let t93690 = t7696 * t26714;
    let t93694 = t7696 * t26717;
    let t93704 = t2173 * t93637;
    let t93709 = t10466 * t3489;
    (t93661, t93662, t93664, t93686, t93690, t93694, t93704, t93709)
}
