//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1231/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1231(t18985: f64, t5745: f64, t1838: f64, t3326: f64, t520: f64, t18979: f64, t1773: f64, t18947: f64, t522: f64, t1266: f64, t1772: f64, t1842: f64, t18481: f64, t18483: f64, t18496: f64, t18948: f64, t18950: f64, t18964: f64, t18968: f64, t18972: f64, t18976: f64, t18981: f64, t3367: f64, t3385: f64, t538: f64, t5737: f64, t5739: f64, t5921: f64, t5925: f64, t5930: f64, t5933: f64) -> (f64, f64, f64, f64, f64) {
    let t18986 = t5745 * t18985;
    let t18991 = t5745 * t1838 * t3326 * t520;
    let t18994 = t5745 * t18979 * t520;
    let t18997 = t1773 * t522 * t18947;
    let t18999 = -2.0_f64 * t1266 * t18950 - t1772 * t18997 - t1842 * t18481 + 4.0_f64 * t18483 * t5925 + 2.0_f64 * t18483 * t5930 - 4.0_f64 * t18496 * t18968 + t18948 * t538 - 6.0_f64 * t18964 * t5739 + 4.0_f64 * t18972 * t5739 + 2.0_f64 * t18976 * t5739 - 2.0_f64 * t18981 * t5739 + 2.0_f64 * t18986 * t5739 + t18991 * t5739 + t18994 * t5739 + 2.0_f64 * t3367 * t5921 - t3385 * t5921 - 2.0_f64 * t5737 * t5933;
    (t18986, t18991, t18994, t18997, t18999)
}
