//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1270/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1270<F: Float>(t33234: F, t9721: F, t9733: F, t2803: F, t48448: F, t79: F, t33162: F, t33183: F, t18325: F, t5445: F, t780: F, t33197: F, t5014: F, t112772: F, t9725: F, t33172: F, t4998: F, t9740: F) -> (F, F, F, F, F, F, F, F) {
    let t113097 = t9721 * t33234;
    let t113099 = t9733 * t33234;
    let t113111 = t48448 * t79 * t2803;
    let t113114 = t33183 * t33162;
    let t113123 = t5445 * t780 * t18325;
    let t113124 = t5014 * t33197;
    let t113134 = t9725 * t112772;
    let t113152 = t9740 * t4998 * t33172;
    (t113097, t113099, t113111, t113114, t113123, t113124, t113134, t113152)
}
