//! MGGA_C_REVTPSS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1096/1428 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part4_v3rho3_1_chunk1096<F: Float>(t2349: F, t97: F, t2255: F, t658: F, t2256: F, t4269: F, t100: F, t580: F, t22: F, t4273: F, t10241: F, t1509: F, t2358: F) -> (F, F, F, F, F, F) {
    let t13475 = t97 * t2349;
    let t13476 = t2255 * t658;
    let t13479 = t4269 * t2256;
    let t13482 = t100 * t580;
    let t13485 = t4273 * t22;
    let t13493 = t10241 * t1509 * t2358;
    (t13475, t13476, t13479, t13482, t13485, t13493)
}
