//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 614/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk614(t460: f64, t5462: f64, t3302: f64, t3603: f64, t1248: f64, t5332: f64, t1269: f64, t1287: f64, t1794: f64, t487: f64, t5284: f64, t3781: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t5463 = t460 * t5462;
    let t5464 = t3302 * t3603;
    let t5465 = t5464 * t1248;
    let t5466 = t5332 * t5465;
    let t5470 = t1269 * t1794 * t1287;
    let t5474 = t487 * t5284 * t1287;
    let t5477 = t3781 * t487;
    (t5463, t5465, t5466, t5470, t5474, t5477)
}
