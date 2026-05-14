//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1255/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1255<F: Float>(t35421: F, t35457: F, t35491: F, t35519: F, t10028: F, t10039: F, t18925: F, t2042: F, t25153: F, t2666: F, t2815: F, t34377: F, t35269: F, t35273: F, t35344: F, t35374: F, t35378: F, t5532: F, t7656: F, t802: F, t9291: F, t9760: F) -> (F, F) {
    let t35521 = t35421 + t35457 + t35491 + t35519;
    let t35523 = 4.0 * t10028 * t18925 - 2.0 * t10039 * t7656 - t2042 * t35374 - t25153 * t2815 - 2.0 * t2666 * t34377 + 2.0 * t35344 * t5532 + 4.0 * t35378 * t5532 + t35521 * t802 - t9291 * t9760 + t35269 + t35273;
    (t35521, t35523)
}
