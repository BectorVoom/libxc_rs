//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1348/1439 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1348(t1445: f64, t1562: f64, t2293: f64, t8097: f64, t2854: f64, t6393: f64, t10448: f64, t4953: f64, t3338: f64, t4529: f64, t10374: f64, t4614: f64, t574: f64) -> (f64, f64, f64, f64, f64) {
    let t34032 = 0.13803453343411469884e2_f64 * t1562 * t1445 * t8097 * t2293;
    let t34036 = 0.69017266717057349418e1_f64 * t1562 * t1445 * t2854 * t6393;
    let t34038 = 0.13803453343411469884e2_f64 * t4953 * t10448;
    let t34045 = t4529 * t3338;
    let t34052 = 0.12269736305254639897e2_f64 * t574 * t4614 * t10374;
    (t34032, t34036, t34038, t34045, t34052)
}
