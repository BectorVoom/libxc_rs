//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1256/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1256(t2822: f64, t28924: f64, t1020: f64, t19135: f64, t26760: f64, t19140: f64, t4994: f64, t19785: f64, t1092: f64, t19715: f64, t27763: f64, t19576: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t100514 = t2822 * t28924;
    let t100519 = t1020 * t26760 * t19135;
    let t100522 = t4994 * t26760 * t19140;
    let t100525 = t1020 * t26760 * t19785;
    let t100528 = t1092 * t27763 * t19715;
    let t100531 = t1092 * t26760 * t19576;
    (t100514, t100519, t100522, t100525, t100528, t100531)
}
