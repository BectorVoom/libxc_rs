//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1010/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1010(t3205: f64, t5451: f64, t1270: f64, t1268: f64, t1625: f64, t10038: f64, t10042: f64, t1206: f64, t12913: f64, t12915: f64, t12918: f64, t12924: f64, t13812: f64, t13813: f64, t13814: f64, t13815: f64, t13816: f64, t13817: f64, t3183: f64, t3184: f64, t4519: f64, t4524: f64, t4525: f64, t5366: f64, t7979: f64, t7988: f64, t7992: f64) -> (f64, f64, f64) {
    let t13955 = t5451 * t3205;
    let t13958 = t5451 * t1270;
    let t13965 = t1625 * t1268;
    let t13972 = 3.0_f64 * t1206 * t13958 * t3183 - t1268 * t13955 * t4524 - 6.0_f64 * t13965 * t3183 * t4525 + 3.0_f64 * t3183 * t3184 * t5366 - 2.0_f64 * t4519 * t4524 * t4525 - t10038 - t10042 - t12913 - t12915 + t12918 - t12924 + t13812 - t13813 + t13814 + t13815 - t13816 - t13817 + t7979 + t7988 + t7992;
    (t13955, t13965, t13972)
}
