//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 824/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk824(t4873: f64, t5039: f64, t7097: f64, t7126: f64, t7156: f64, t8646: f64, t8647: f64, t8649: f64, t8651: f64, t8652: f64, t8653: f64, t8654: f64, t8655: f64, t8656: f64, t8657: f64, t8658: f64) -> f64 {
    let t8659 = -t7097 + t8646 - t8647 + t8649 - t8651 + t7126 + t8652 - t8653 - t8654 - t8655 + t4873 - t7156 - t8656 - t8657 + t8658 + t5039;
    t8659
}
