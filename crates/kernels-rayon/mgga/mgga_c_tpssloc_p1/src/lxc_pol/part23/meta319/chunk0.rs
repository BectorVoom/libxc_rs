//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1079/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1079(t22132: f64, t974: f64, t11759: f64, t20234: f64, t21745: f64, t3440: f64, t11649: f64, t1174: f64, t1726: f64, t18310: f64, t18312: f64, t18314: f64, t18321: f64, t18325: f64, t18327: f64, t18330: f64, t18333: f64, t22012: f64, t22015: f64, t22116: f64, t22119: f64, t22129: f64, t488: f64, t4889: f64, t6178: f64, t6184: f64, t6188: f64) -> (f64, f64, f64, f64, f64) {
    let t22133 = t974 * t22132;
    let t22136 = t11759 * t20234;
    let t22137 = t974 * t22136;
    let t22149 = t3440 * t21745;
    let t22152 = -7.0_f64 / 648.0_f64 * t1174 * t22012 - t22015 * t488 / 192.0_f64 + t22116 * t488 / 3072.0_f64 - t1174 * t22119 / 48.0_f64 + t11649 - t4889 * t6178 / 27.0_f64 + t4889 * t6184 / 36.0_f64 + t4889 * t6188 / 18.0_f64 - t1174 * t22129 / 288.0_f64 - t1174 * t22133 / 48.0_f64 + t1174 * t22137 / 36.0_f64 + t18310 / 1536.0_f64 - t18312 / 144.0_f64 + 19.0_f64 / 864.0_f64 * t18314 - t18325 / 144.0_f64 + t18327 / 54.0_f64 - t18330 / 288.0_f64 + t18333 / 216.0_f64 - 11.0_f64 / 108.0_f64 * t18321 * t1726 + t1174 * t22149 / 72.0_f64;
    (t22133, t22136, t22137, t22149, t22152)
}
