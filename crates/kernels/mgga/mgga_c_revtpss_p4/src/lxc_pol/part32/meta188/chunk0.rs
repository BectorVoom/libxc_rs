//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 847/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk847<F: Float>(t1699: F, t3336: F, t1100: F, t1102: F, t198: F, t336: F, t4589: F, t4592: F, t4594: F, t4597: F, t4634: F, t4638: F, t4716: F, t4718: F, t4721: F, t4723: F, t4727: F, t4731: F, t4736: F, t5019: F, t5023: F) -> (F, F) {
    let t5024 = t1699 * t3336;
    let t5027 = t1102 * t198 * t336 * t5019 - t1100 * t5023 * t5024 - t4589 + t4592 + t4594 - t4597 + t4634 + t4638 + t4716 + t4718 - t4721 - t4723 + t4727 - t4731 - t4736;
    (t5024, t5027)
}
