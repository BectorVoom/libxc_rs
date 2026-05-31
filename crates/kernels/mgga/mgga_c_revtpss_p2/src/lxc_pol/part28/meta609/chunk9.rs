//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2128/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2128<F: Float>(t2322: F, t2371: F, t25078: F, t25805: F, t28050: F, t4248: F, t4254: F, t4257: F, t4292: F, t651: F, t7221: F, t7883: F, t97639: F, t97641: F, t97643: F, t97645: F, t97647: F, t97649: F, t97653: F, t97657: F, t97659: F, t97661: F, t97663: F, t97666: F, t98421: F) -> F {
    let t98422 = -F::cast_from(2.0_f64) * t2371 * t651 * t7883 - F::cast_from(4.0_f64) * t4292 * t651 * t7221 - F::cast_from(4.0_f64) * t2322 * t28050 - F::cast_from(2.0_f64) * t25078 * t4248 - F::cast_from(4.0_f64) * t25805 * t4257 - F::cast_from(4.0_f64) * t28050 * t4254 - t97639 - t97641 - t97643 - t97645 - t97647 - t97649 + t97653 + t97657 + t97659 + t97661 - t97663 - t97666 + t98421;
    t98422
}
