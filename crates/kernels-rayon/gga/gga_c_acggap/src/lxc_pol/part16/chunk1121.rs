//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 1121/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk1121(t142: f64, t6313: f64, t8888: f64, t2020: f64, t9761: f64, t7839: f64, t9633: f64, t2068: f64, t2263: f64, t35137: f64, t8480: f64, t8521: f64) -> (f64, f64, f64, f64, f64) {
    let t39525 = t8888 * t142 * t6313;
    let t39527 = t2020 * t9761;
    let t39534 = t7839 * t9633;
    let t39537 = t2068 * t35137 * t2263;
    let t39540 = t2068 * t8480 * t8521;
    (t39525, t39527, t39534, t39537, t39540)
}
