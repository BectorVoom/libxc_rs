//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 837/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk837(t1457: f64, t8000: f64, t8004: f64, t1305: f64, t2787: f64, t1445: f64, t1555: f64, t999: f64, t2822: f64, t528: f64, t1564: f64, t2754: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t8077 = t1457 * t8000;
    let t8080 = t1457 * t8004;
    let t8083 = t2787 * t1305;
    let t8084 = t1445 * t8083;
    let t8087 = t1555 * t999;
    let t8090 = t528 * t2822;
    let t8097 = t1564 * t2754;
    (t8077, t8080, t8084, t8087, t8090, t8097)
}
