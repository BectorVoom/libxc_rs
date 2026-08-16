//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 844/1468 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk844(t2779: f64, t4614: f64, t1323: f64, t2787: f64, t1445: f64, t1603: f64, t999: f64, t1457: f64, t7957: f64, t493: f64, t7892: f64, t590: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t8176 = t4614 * t2779;
    let t8179 = t2787 * t1323;
    let t8180 = t1445 * t8179;
    let t8183 = t1603 * t999;
    let t8190 = t1457 * t7957;
    let t8195 = t493 * t7892;
    let t8196 = t8195 * t590;
    (t8176, t8180, t8183, t8190, t8195, t8196)
}
