//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 1428/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1428(t13652: f64, t1317: f64, t5569: f64, t3829: f64, t566: f64, t13640: f64, t13641: f64, t13643: f64, t13644: f64, t13645: f64, t13646: f64, t13647: f64, t13648: f64, t1448: f64, t1868: f64, t198: f64, t4139: f64, t4140: f64, t5541: f64, t5591: f64, t9514: f64, t9517: f64, t9521: f64, t9555: f64, t9569: f64, t9574: f64, t9577: f64, t9588: f64) -> (f64, f64, f64) {
    let t13653 = 0.17315859105681463759e2_f64 * t13652;
    let t13654 = t1317 * t5569;
    let t13655 = 8.0_f64 * t13654;
    let t13656 = t3829 * t566;
    let t13663 = -2.0_f64 * t13648 * t1448 * t5541 + 6.0_f64 * t13656 * t1868 * t198 + 6.0_f64 * t4139 * t4140 * t5591 - t13640 + t13641 + t13643 - t13644 + t13645 - t13646 - t13647 - t13653 + t13655 + t9514 - t9517 - t9521 + t9555 + t9569 - t9574 - t9577 - t9588;
    (t13653, t13655, t13663)
}
