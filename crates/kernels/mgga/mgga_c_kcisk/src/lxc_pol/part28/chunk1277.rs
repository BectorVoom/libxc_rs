//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1277/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1277<F: Float>(t1101: F, t110994: F, t9368: F, t15492: F, t32646: F, t32643: F, t32669: F, t32652: F, t15692: F, t397: F, t9366: F, t9365: F, t32636: F, t3368: F, t140: F, t15430: F, t190: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t110995 = t1101 * t110994;
    let t110996 = t110995 * t9368;
    let t110999 = t15492 * t32646 * t9368;
    let t111001 = t32669 * t32643;
    let t111003 = t32652 * t32643;
    let t111006 = t397 * t9366 * t15692;
    let t111007 = t9365 * t111006;
    let t111009 = t3368 * t32636;
    let t111010 = t111009 * t9368;
    let t111013 = t140 * t15430 * t190;
    (t110995, t110996, t110999, t111001, t111003, t111006, t111007, t111009, t111010, t111013)
}
