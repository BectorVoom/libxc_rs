//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1246/1439 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1246(t10826: f64, t1841: f64, t2536: f64, t734: f64, t1944: f64, t3444: f64, t29478: f64, t29480: f64, t29483: f64, t29486: f64, t29489: f64, t32604: f64, t32610: f64, t32615: f64, t32618: f64, t32621: f64, t32623: f64, t32625: f64, t32629: f64) -> f64 {
    let t32633 = 0.17090058289204942853e-2_f64 * t1841 * t2536 * t10826 * t734;
    let t32634 = t1944 * t3444;
    let t32635 = 0.99692006687028833308e-3_f64 * t32634;
    let t32636 = -t32604 - t32610 + t32615 + t32618 + t32621 - t32623 - t32625 - t32629 - t32633 - t29478 + t29480 + t29483 + t29486 - t29489 - t32635;
    t32636
}
