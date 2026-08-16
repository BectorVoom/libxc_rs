//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 879/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk879(t18594: f64, t18690: f64, t18772: f64, t19110: f64, t1022: f64, t1096: f64, t1092: f64, t4985: f64, t5026: f64, t4814: f64, t4999: f64, t5005: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t19112 = t18594 + t18690 + t18772 + t19110;
    let t19113 = t1022 * t19112;
    let t19114 = t1096 * t19113;
    let t19115 = t1092 * t19114;
    let t19117 = t5026 * t4985;
    let t19118 = t1092 * t19117;
    let t19120 = t4999 * t4814;
    let t19121 = t1092 * t19120;
    let t19123 = t4999 * t5005;
    (t19112, t19113, t19115, t19118, t19121, t19123)
}
