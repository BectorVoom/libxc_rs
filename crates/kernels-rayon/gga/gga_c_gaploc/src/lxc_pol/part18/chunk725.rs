//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 725/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk725(t1445: f64, t6784: f64, t6424: f64, t2389: f64, t2410: f64, t1457: f64, t6443: f64, t2335: f64, t4673: f64, t2398: f64, t4614: f64, t2378: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t6785 = t1445 * t6784;
    let t6790 = t1445 * t6424;
    let t6793 = t2410 * t2389;
    let t6795 = t1457 * t6443;
    let t6798 = t4673 * t2335;
    let t6801 = t4614 * t2398;
    let t6804 = t4614 * t2378;
    (t6785, t6790, t6793, t6795, t6798, t6801, t6804)
}
