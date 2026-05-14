//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1315/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1315<F: Float>(t112551: F, t113037: F, t9725: F, t18325: F, t5445: F, t780: F, t33197: F, t5014: F, t1994: F) -> (F, F, F, F, F) {
    let t113058 = 0.51588271604938271604e-3 * t112551;
    let t113069 = t9725 * t113037;
    let t113123 = t5445 * t780 * t18325;
    let t113124 = t5014 * t33197;
    let t113181 = t1994 * t780 * t18325;
    (t113058, t113069, t113123, t113124, t113181)
}
