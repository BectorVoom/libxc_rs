//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2227/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2227(t135: f64, t6187: f64, t1174: f64, t4889: f64, t5040: f64, t6183: f64, t6177: f64, t1198: f64, t15484: f64, t15488: f64, t15490: f64, t15494: f64, t15498: f64, t15524: f64, t15550: f64, t15574: f64, t15580: f64, t15737: f64, t1748: f64, t18321: f64, t4980: f64, t5024: f64, t5030: f64) -> f64 {
    let t18324 = t135 * t6187;
    let t18325 = t1174 * t18324;
    let t18327 = t4889 * t5040;
    let t18329 = t135 * t6183;
    let t18330 = t1174 * t18329;
    let t18332 = t135 * t6177;
    let t18333 = t1174 * t18332;
    let t18337 = t15498 * t1748 / 432.0_f64 + t5024 * t5030 / 432.0_f64 - t15484 - t15488 + t15490 + t15494 + t15524 - 11.0_f64 / 324.0_f64 * t18321 * t1198 - t18325 / 432.0_f64 + t18327 / 162.0_f64 - t18330 / 864.0_f64 - t15550 - t15574 + t18333 / 648.0_f64 - t15580 + t15737 * t4980 / 768.0_f64;
    t18337
}
