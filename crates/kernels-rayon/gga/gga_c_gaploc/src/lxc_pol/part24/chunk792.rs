//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 792/1439 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk792(t1835: f64, t2581: f64, t1445: f64, t2066: f64, t954: f64, t7250: f64, t7254: f64, t2645: f64, t4614: f64, t2572: f64, t4673: f64, t1865: f64, t2571: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t7464 = t2581 * t1835;
    let t7465 = t1445 * t7464;
    let t7468 = t2066 * t954;
    let t7473 = t1445 * t7250;
    let t7476 = t1445 * t7254;
    let t7479 = t4614 * t2645;
    let t7482 = t4673 * t2572;
    let t7487 = t2571 * t1865;
    (t7465, t7468, t7473, t7476, t7479, t7482, t7487)
}
