//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 1681/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1681<F: Float>(t20248: F, t21657: F, t118: F, t1310: F, t13426: F, t1502: F, t1519: F, t18220: F, t18227: F, t18232: F, t18235: F, t18242: F, t18245: F, t1843: F, t2322: F, t4246: F, t4248: F, t4254: F, t4257: F, t508: F, t5517: F, t5877: F, t5884: F, t5921: F, t651: F, t671: F) -> (F, F) {
    let t21658 = t20248 + t21657;
    let t21660 = -t118 * t21658 - t1310 * t5877 - F::new(2.0) * t1310 * t5884 - F::new(4.0) * t13426 * t1519 - F::new(2.0) * t1502 * t5517 - F::new(4.0) * t1519 * t18227 - F::new(2.0) * t18220 * t508 - F::new(2.0) * t18232 * t651 - F::new(4.0) * t18235 * t651 - F::new(2.0) * t18242 * t651 - F::new(2.0) * t18245 * t671 - F::new(2.0) * t1843 * t4246 - F::new(2.0) * t2322 * t5921 - F::new(4.0) * t4248 * t4257 - F::new(2.0) * t4254 * t5921;
    (t21658, t21660)
}
