//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1077/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1077<F: Float>(t21700: F, t21720: F, t21735: F, t21769: F, t1137: F, t6167: F, t13787: F, t1788: F, t3409: F, t5991: F, t1083: F, t1131: F, t1298: F, t13230: F, t13235: F, t13253: F, t15814: F, t16745: F, t1772: F, t21663: F, t335: F, t336: F, t367: F, t368: F, t372: F, t398: F, t418: F, t4256: F, t4630: F, t5641: F) -> (F, F) {
    let t21771 = t21700 + t21720 + t21735 + t21769;
    let t21776 = t1137 * t6167;
    let t21778 = t13787 * t1788;
    let t21790 = t3409 * t5991;
    let t21795 = 0.68598428988911579156e-2 * t21663 + t15814 * t4256 * t5641 * t372 / 2.0 - t367 * t336 * t368 * t21771 / 96.0 + 7.0 / 72.0 * t21776 + 35.0 / 72.0 * t21778 - t335 * t336 * t4630 * t1298 / 12.0 + 0.20007875121765877254e-2 * t16745 - 0.85748036236139473944e-3 * t418 * t398 * t1083 * t1772 * t1131 - 0.16006300097412701803e-1 * t21790 - 0.17149607247227894789e-2 * t13230 - 0.85748036236139473944e-3 * t13235 + 0.17149607247227894789e-2 * t13253;
    (t21771, t21795)
}
