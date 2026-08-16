//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1745/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1745(t1042: f64, t17505: f64, t1797: f64, t21107: f64, t24612: f64, t3610: f64, t3611: f64, t5268: f64, t5296: f64, t5384: f64, t5825: f64, t6573: f64, t6625: f64, t6631: f64, t6635: f64, t71693: f64, t71699: f64, t82555: f64, t82821: f64, t82824: f64, t82827: f64, t90037: f64, t90081: f64) -> f64 {
    let t90245 = 0.86891343385954666928e-1_f64 * t71693 * t6631 - 0.34299214494455789578e-2_f64 * t5384 * t1042 * t5268 * t90037 - 0.17149607247227894789e-2_f64 * t5384 * t1042 * t5296 * t5825 * t6573 - 0.18292914397043087775e-1_f64 * t17505 * t24612 - 0.43445671692977333464e-1_f64 * t71699 * t6635 - 0.13719685797782315831e-1_f64 * t82555 * t1797 - 0.64311027177104605458e-3_f64 * t3610 * t1042 * t90081 * t3611 - 0.13719685797782315831e-1_f64 * t21107 * t6625 - 0.18292914397043087775e-1_f64 * t82821 + 0.11433071498151929859e-2_f64 * t82824 + 0.19055119163586549765e-2_f64 * t82827;
    t90245
}
