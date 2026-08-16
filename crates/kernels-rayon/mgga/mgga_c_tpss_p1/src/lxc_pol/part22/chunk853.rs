//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 853/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk853(t1265: f64, t1838: f64, t5740: f64, t1232: f64, t520: f64, t5745: f64, t1773: f64, t522: f64, t5918: f64, t1266: f64, t1772: f64, t1842: f64, t538: f64, t5737: f64, t5739: f64, t5919: f64, t5921: f64) -> (f64, f64, f64, f64) {
    let t5924 = t1838 * t1265;
    let t5925 = t5740 * t5924;
    let t5930 = t5745 * t1838 * t1232 * t520;
    let t5933 = t1773 * t522 * t5918;
    let t5935 = -t1266 * t5921 - t1772 * t5933 - t1842 * t5737 + t538 * t5919 + 2.0_f64 * t5739 * t5925 + t5739 * t5930;
    (t5925, t5930, t5933, t5935)
}
