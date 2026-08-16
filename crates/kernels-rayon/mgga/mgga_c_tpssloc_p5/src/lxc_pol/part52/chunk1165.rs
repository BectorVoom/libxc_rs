//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1165/1400 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1165(t1874: f64, t26103: f64, t6517: f64, t6525: f64, t532: f64, t8492: f64, t1307: f64, t3701: f64, t1983: f64, t6876: f64, t8490: f64, t2015: f64, t3886: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t31080 = t26103 * t1874;
    let t31082 = t6517 * t6525;
    let t31084 = t532 * t8492;
    let t31085 = t3701 * t1307;
    let t31086 = t31084 * t31085;
    let t31088 = 3.0_f64 * t1983 * t31086;
    let t31089 = t6876 * t8490;
    let t31090 = t3886 * t2015;
    (t31080, t31082, t31084, t31086, t31088, t31089, t31090)
}
