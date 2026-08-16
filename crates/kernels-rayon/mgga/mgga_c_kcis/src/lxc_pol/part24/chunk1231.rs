//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1231/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1231(t15216: f64, t29122: f64, t26960: f64, t20330: f64, t5310: f64, t922: f64, t1262: f64, t26996: f64, t5329: f64, t6842: f64, t1020: f64, t26753: f64, t6625: f64) -> (f64, f64, f64, f64, f64) {
    let t100074 = t15216 * t29122;
    let t100075 = t26960 * t100074;
    let t100078 = t5310 * t20330 * t922;
    let t100090 = t5329 * t26996 * t6842 * t1262;
    let t100094 = t1020 * t26753 * t6625;
    (t100074, t100075, t100078, t100090, t100094)
}
