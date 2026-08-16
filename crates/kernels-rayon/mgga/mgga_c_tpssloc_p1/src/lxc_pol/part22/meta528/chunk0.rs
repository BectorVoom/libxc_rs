//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1998/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1998(t1215: f64, t1409: f64, t254: f64, t492: f64, t1254: f64, t1763: f64, t1441: f64, t1458: f64, t343: f64, t5842: f64, t5456: f64, t576: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t27524 = t1409 * t1215;
    let t27784 = t492 * t254;
    let t27843 = t1763 * t1254;
    let t28002 = t1441 * t1458;
    let t28565 = t5842 * t343;
    let t28893 = t576 * t5456;
    (t27524, t27784, t27843, t28002, t28565, t28893)
}
