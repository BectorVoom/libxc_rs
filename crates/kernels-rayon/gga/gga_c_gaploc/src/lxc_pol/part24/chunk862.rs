//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 862/1439 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk862(t1890: f64, t8502: f64, t590: f64, t1392: f64, t2949: f64, t1391: f64, t1835: f64, t1445: f64, t1980: f64, t2975: f64, t2925: f64, t296: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t8503 = t1890 * t8502;
    let t8504 = t8503 * t590;
    let t8508 = t1392 * t2949;
    let t8509 = t1391 * t8508;
    let t8512 = t2949 * t1835;
    let t8513 = t1445 * t8512;
    let t8516 = t1980 * t2975;
    let t8519 = t296 * t2925;
    (t8504, t8509, t8512, t8513, t8516, t8519)
}
