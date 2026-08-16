//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 755/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk755(t147: f64, t21099: f64, t4917: f64, t9490: f64, t9498: f64, t2321: f64, t4635: f64, t231: f64, t5053: f64, t1526: f64, t17685: f64, t17703: f64, t2320: f64, t342: f64, t343: f64, t3806: f64, t4915: f64, t4922: f64, t5059: f64, t9482: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t148 = 10000000.0_f64 <= t147;
    let t21100 = piecewise3(t148, 0.0_f64, t21099);
    let t21103 = t9490 * t4917;
    let t21110 = t9498 * t4917;
    let t21114 = t2321 * t4635;
    let t21118 = t231 * t5053;
    let t21122 = t4915 + t5059 + t9482 - t17685 / 18.0_f64 - t17703 / 6.0_f64 - t1526 * t3806 * t21103 / 9.0_f64 - t1526 * t2320 * t4922 / 6.0_f64 + t1526 * t2320 * t21110 / 6.0_f64 - t1526 * t2320 * t21114 / 12.0_f64 - t342 * t343 * t21118 / 4.0_f64;
    (t21100, t21103, t21110, t21114, t21118, t21122)
}
