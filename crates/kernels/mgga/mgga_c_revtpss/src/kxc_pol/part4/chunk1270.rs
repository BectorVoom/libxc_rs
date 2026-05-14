//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1270/1274 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk1270<F: Float>(t16630: F, t18152: F, t2371: F, t94: F, t118: F, t1310: F, t1315: F, t13425: F, t13426: F, t13429: F, t14310: F, t1519: F, t1843: F, t1847: F, t1911: F, t2320: F, t2322: F, t2331: F, t3821: F, t4151: F, t4246: F, t4248: F, t4254: F, t4257: F, t4293: F, t508: F, t511: F, t5517: F, t5787: F, t649: F, t671: F) -> (F,) {
    let t18153 = t16630 + t18152;
    let t18163 = t94 * t2371;
    let t18176 = -t118 * t18153 - 2.0 * t1310 * t4246 + 2.0 * t1315 * t5787 - t13425 * t508 - 4.0 * t13426 * t671 - 2.0 * t13429 * t508 + t14310 * t511 - 2.0 * t1519 * t18163 - t1843 * t2320 + t1847 * t4151 + t1911 * t3821 - 4.0 * t2322 * t4293 - 4.0 * t2331 * t4248 - 4.0 * t4254 * t4257 - 2.0 * t5517 * t649;
    (t18176,)
}
