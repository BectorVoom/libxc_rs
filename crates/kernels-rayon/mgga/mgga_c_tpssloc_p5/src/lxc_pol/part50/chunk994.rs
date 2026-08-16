//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 994/1294 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk994(t22635: f64, t26215: f64, t22633: f64, t2006: f64, t5210: f64, t1807: f64, t6955: f64, t22646: f64, t26184: f64, t26187: f64, t26191: f64, t26195: f64, t26198: f64, t26200: f64, t26204: f64, t26207: f64, t26212: f64, t568: f64) -> (f64, f64, f64) {
    let t26216 = t22635 * t26215;
    let t26217 = t22633 * t26216;
    let t26219 = t5210 * t2006;
    let t26221 = t1807 * t6955;
    let t26223 = 0.38381794893125283518e-1_f64 * t26184 - 0.16449340668482264365e-1_f64 * t26187 - 0.16449340668482264365e-1_f64 * t26191 - 0.16449340668482264365e-1_f64 * t26195 + 0.82246703342411321825e-2_f64 * t26198 + 0.19190897446562641759e-1_f64 * t26200 - 0.82246703342411321825e-2_f64 * t26204 - 0.82246703342411321825e-2_f64 * t26207 + 0.82246703342411321825e-2_f64 * t26212 + 0.16449340668482264365e-1_f64 * t26217 - t22646 + t26219 * t568 + t26221 * t568;
    (t26219, t26221, t26223)
}
