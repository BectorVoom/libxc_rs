//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 1082/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk1082(t30775: f64, t30777: f64, t2290: f64, t7630: f64, t1549: f64, t30540: f64, t1554: f64, t1558: f64, t4695: f64, t7822: f64, t4335: f64, t2068: f64, t4680: f64, t8521: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t34847 = 0.17149607247227894789e-2_f64 * t30775;
    let t34848 = 0.17149607247227894789e-2_f64 * t30777;
    let t34849 = t7630 * t2290;
    let t34851 = t30540 * t1549;
    let t34853 = t30540 * t1554;
    let t34855 = t30540 * t1558;
    let t34856 = 0.40015750243531754508e-2_f64 * t34855;
    let t34857 = t7822 * t4695;
    let t34859 = t7822 * t4335;
    let t34862 = t2068 * t4680 * t8521;
    (t34847, t34848, t34849, t34851, t34853, t34856, t34857, t34859, t34862)
}
