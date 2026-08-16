//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1118/1428 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk1118<F: Float>(t5651: F, t808: F, t9736: F, t241: F, t820: F, t9991: F, t3923: F, t9994: F, t5673: F, t5674: F, t5697: F, t9962: F) -> (F, F, F, F, F) {
    let t13800 = t808 * t5651;
    let t13801 = t9736 * t13800;
    let t13804 = t820 * t9991 * t241;
    let t13805 = t9994 * t3923;
    let t13807 = t5673 * t5674 * t13805;
    let t13810 = t9962 * t5697;
    (t13801, t13804, t13805, t13807, t13810)
}
