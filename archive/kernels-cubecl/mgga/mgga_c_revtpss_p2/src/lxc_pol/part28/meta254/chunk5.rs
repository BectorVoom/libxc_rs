//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 1130/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1130<F: Float>(t5782: F, t5786: F, t118: F, t1310: F, t1315: F, t1453: F, t1502: F, t1519: F, t1843: F, t1847: F, t1911: F, t2322: F, t4246: F, t4248: F, t4254: F, t4257: F, t4293: F, t4297: F, t508: F, t511: F, t5517: F, t5528: F, t569: F, t649: F, t651: F, t671: F) -> (F, F) {
    let t5787 = t5782 + t5786;
    let t5789 = -t118 * t5517 - t1310 * t1502 + t1315 * t1911 + t1453 * t1847 - F::cast_from(2.0_f64) * t1519 * t2322 - F::cast_from(2.0_f64) * t1519 * t4254 - t1843 * t649 - t4246 * t508 - F::cast_from(2.0_f64) * t4248 * t671 - F::cast_from(2.0_f64) * t4257 * t651 - F::cast_from(2.0_f64) * t4293 * t651 - F::cast_from(2.0_f64) * t4297 * t651 + t511 * t5787 + t5528 * t569;
    (t5787, t5789)
}
