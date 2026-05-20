//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1255/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1255<F: Float>(t12663: F, t13189: F, t12413: F, t12417: F, t12566: F, t12573: F, t12575: F, t12577: F, t12579: F, t12583: F, t12584: F, t12587: F, t12594: F, t12598: F, t1298: F, t1300: F, t198: F, t336: F, t3794: F, t3801: F, t5023: F) -> (F, F) {
    let t13190 = t12663 + t13189;
    let t13194 = F::new(2.0) * t12584 * t12587 * t198 * t336 - F::new(3.0) * t1298 * t3794 * t3801 * t5023 + t1300 * t13190 * t198 * t336 - t12413 + t12417 - t12566 - t12573 - t12575 - t12577 + t12579 + t12583 - t12594 - t12598;
    (t13190, t13194)
}
