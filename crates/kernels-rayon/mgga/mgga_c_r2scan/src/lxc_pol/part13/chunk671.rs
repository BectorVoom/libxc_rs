//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 671/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk671(t37: f64, t4888: f64, t89: f64, t1377: f64, t406: f64, t410: f64, t1422: f64, t458: f64, t1419: f64, t425: f64, t1416: f64, t44: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t4889 = t37 * t4888;
    let t4890 = t4889 * t89;
    let t4891 = 120.0_f64 * t4890;
    let t4892 = t406 * t1377;
    let t4894 = t410 * t1377;
    let t4896 = t1422 * t458;
    let t4898 = t1419 * t425;
    let t4900 = t1416 * t458;
    let t4901 = 60.0_f64 * t4900;
    let t4902 = t44 * t44;
    (t4889, t4891, t4892, t4894, t4896, t4898, t4901, t4902)
}
