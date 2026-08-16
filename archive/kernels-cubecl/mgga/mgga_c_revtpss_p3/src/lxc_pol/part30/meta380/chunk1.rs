//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 1428/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1428<F: Float>(t13652: F, t1317: F, t5569: F, t3829: F, t566: F, t13640: F, t13641: F, t13643: F, t13644: F, t13645: F, t13646: F, t13647: F, t13648: F, t1448: F, t1868: F, t198: F, t4139: F, t4140: F, t5541: F, t5591: F, t9514: F, t9517: F, t9521: F, t9555: F, t9569: F, t9574: F, t9577: F, t9588: F) -> (F, F, F) {
    let t13653 = F::cast_from(0.17315859105681463759e2_f64) * t13652;
    let t13654 = t1317 * t5569;
    let t13655 = F::cast_from(8.0_f64) * t13654;
    let t13656 = t3829 * t566;
    let t13663 = -F::cast_from(2.0_f64) * t13648 * t1448 * t5541 + F::cast_from(6.0_f64) * t13656 * t1868 * t198 + F::cast_from(6.0_f64) * t4139 * t4140 * t5591 - t13640 + t13641 + t13643 - t13644 + t13645 - t13646 - t13647 - t13653 + t13655 + t9514 - t9517 - t9521 + t9555 + t9569 - t9574 - t9577 - t9588;
    (t13653, t13655, t13663)
}
