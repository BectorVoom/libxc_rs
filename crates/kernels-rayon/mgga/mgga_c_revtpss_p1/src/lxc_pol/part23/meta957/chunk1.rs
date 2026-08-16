//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3202/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3202(t59337: f64, t59339: f64, t71827: f64, t71845: f64, t71859: f64, t71880: f64, t71883: f64, t71886: f64, t71908: f64, t71920: f64, t71928: f64, t1256: f64, t24684: f64) -> (f64, f64) {
    let t84078 = -0.28582678745379824648e-3_f64 * t71827 + 0.17149607247227894789e-2_f64 * t71845 + 0.45732285992607719436e-2_f64 * t71859 - 2.0_f64 / 81.0_f64 * t71880 + t71883 / 216.0_f64 + 0.57165357490759649295e-3_f64 * t71886 + t59337 - t59339 - 0.95275595817932748825e-3_f64 * t71908 + 0.45732285992607719436e-2_f64 * t71920 + t71928 / 432.0_f64;
    let t84082 = t24684 * t1256;
    (t84078, t84082)
}
