//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 714/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk714(t1328: f64, t2334: f64, t1445: f64, t2344: f64, t1323: f64, t1603: f64, t894: f64, t2345: f64, t4614: f64, t2417: f64, t1457: f64, t6424: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t6664 = t2334 * t1328;
    let t6665 = t1445 * t6664;
    let t6668 = t2344 * t1328;
    let t6669 = t1445 * t6668;
    let t6672 = t2344 * t1323;
    let t6673 = t1445 * t6672;
    let t6676 = t1603 * t894;
    let t6679 = t4614 * t2345;
    let t6682 = t4614 * t2417;
    let t6689 = t1457 * t6424;
    (t6665, t6669, t6673, t6676, t6679, t6682, t6689)
}
