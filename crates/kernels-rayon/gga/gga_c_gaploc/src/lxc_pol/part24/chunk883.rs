//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 883/1439 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk883(t7585: f64, t8561: f64, t1890: f64, t2925: f64, t590: f64, t1445: f64, t8612: f64, t1628: f64, t3066: f64, t1043: f64, t4598: f64, t1029: f64, t4585: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t8797 = t7585 * t8561;
    let t8802 = t1890 * t2925;
    let t8803 = t8802 * t590;
    let t8806 = t1445 * t8612;
    let t8809 = t1628 * t3066;
    let t8816 = t4598 * t1043;
    let t8819 = t4585 * t1029;
    (t8797, t8802, t8803, t8806, t8809, t8816, t8819)
}
