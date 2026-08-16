//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1201/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1201(t32714: f64, t6936: f64, t1814: f64, t8465: f64, t8467: f64, t5248: f64, t5249: f64, t550: f64, t31170: f64, t1831: f64, t8466: f64, t31137: f64, t7691: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t32715 = t6936 * t32714;
    let t32717 = t1814 * t8465;
    let t32718 = t32717 * t8467;
    let t32721 = t5248 * t5249 * t550;
    let t32722 = t31170 * t32721;
    let t32724 = t8466 * t1831;
    let t32731 = t31137 * t7691;
    (t32715, t32717, t32718, t32721, t32722, t32724, t32731)
}
