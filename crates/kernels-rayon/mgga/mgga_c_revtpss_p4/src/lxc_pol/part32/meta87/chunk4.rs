//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 531/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk531(t265: f64, t502: f64, t1210: f64, t1274: f64, t1770: f64, t1775: f64, t1813: f64, t1829: f64, t460: f64, t495: f64, t1300: f64, t1587: f64, t1721: f64, t1735: f64, t1761: f64, t1763: f64, t1767: f64, t198: f64, t336: f64) -> (f64, f64) {
    let t503 = t265 < t502;
    let t1832 = 0.65854491829355115987e0_f64 * t1770 * t495 - 0.65854491829355115987e0_f64 * t1210 * t1775 + 0.65854491829355115987e0_f64 * t460 * t1813 - 0.65854491829355115987e0_f64 * t1274 * t1829;
    let t1837 = piecewise3(t503, t1300 * t1832 * t198 * t336 - t1721 + t1735 + t1761 + t1763 - t1767, t1587);
    (t1832, t1837)
}
