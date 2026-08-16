//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1257/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1257(t1232: f64, t520: f64, t6419: f64, t5745: f64, t1773: f64, t20154: f64, t522: f64, t1772: f64, t18483: f64, t18496: f64, t19540: f64, t20179: f64, t20183: f64, t20187: f64, t20191: f64, t20196: f64, t20200: f64, t20202: f64, t20206: f64, t5737: f64, t5739: f64, t6430: f64, t6433: f64) -> (f64, f64, f64) {
    let t20210 = t6419 * t1232 * t520;
    let t20211 = t5745 * t20210;
    let t20214 = t1773 * t522 * t20154;
    let t20216 = -t1772 * t20214 + t18483 * t6430 - 2.0_f64 * t18496 * t20187 - 2.0_f64 * t19540 * t20191 + t19540 * t20202 + 2.0_f64 * t20179 * t5739 + 2.0_f64 * t20183 * t5739 + t20196 * t5739 + t20200 * t5739 + 2.0_f64 * t20206 * t5739 + t20211 * t5739 - t5737 * t6433;
    (t20211, t20214, t20216)
}
