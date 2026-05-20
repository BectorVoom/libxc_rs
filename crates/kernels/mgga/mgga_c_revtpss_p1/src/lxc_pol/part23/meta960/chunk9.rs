//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3242/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3242<F: Float>(t117: F, t85307: F, t118: F, t13426: F, t18227: F, t18232: F, t18235: F, t18242: F, t18245: F, t1843: F, t21814: F, t25043: F, t4248: F, t4297: F, t508: F, t5921: F, t649: F, t651: F, t670: F, t671: F, t75931: F, t75941: F, t81110: F, t85032: F) -> (F, F) {
    let t85308 = t85307 * t117;
    let t85312 = -F::new(6.0) * t18245 * t4297 - F::new(2.0) * t651 * t25043 * t670 - F::new(2.0) * t651 * t508 * t75931 - F::new(6.0) * t13426 * t5921 - F::new(6.0) * t18227 * t5921 - F::new(6.0) * t4248 * t18242 - F::new(2.0) * t75941 * t671 - F::new(12.0) * t4248 * t18235 - F::new(6.0) * t4248 * t18232 - t649 * t25043 - t118 * (t81110 + t85032) - t85308 * t508 - F::new(3.0) * t21814 * t1843;
    (t85308, t85312)
}
