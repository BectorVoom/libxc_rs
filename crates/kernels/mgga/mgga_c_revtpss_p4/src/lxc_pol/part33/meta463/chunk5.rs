//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 1686/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1686<F: Float>(t20735: F, t21357: F, t21393: F, t21633: F, t12587: F, t6752: F, t1298: F, t1300: F, t198: F, t20571: F, t20573: F, t20576: F, t20579: F, t20582: F, t20631: F, t20633: F, t20635: F, t20637: F, t20639: F, t20643: F, t20647: F, t20650: F, t20654: F, t20692: F, t20889: F, t20894: F, t20898: F, t336: F, t5023: F) -> (F, F) {
    let t21635 = t20735 + t21357 + t21393 + t21633;
    let t21639 = t6752 * t12587;
    let t21643 = t1300 * t198 * t21635 * t336 - t1298 * t20692 * t5023 + F::cast_from(2.0_f64) * t1298 * t21639 * t5023 - t20571 + t20573 + t20576 - t20579 - t20582 + t20631 + t20633 + t20635 - t20637 + t20639 - t20643 + t20647 + t20650 + t20654 + t20889 - t20894 - t20898;
    (t21635, t21643)
}
