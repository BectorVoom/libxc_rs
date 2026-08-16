//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1412/1439 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1412(t10466: f64, t7014: f64, t20843: f64, t2487: f64, t3395: f64, t10546: f64, t1415: f64, t1645: f64, t4807: f64, t10399: f64, t1436: f64, t18482: f64, t31540: f64) -> (f64, f64, f64, f64, f64) {
    let t34927 = t7014 * t10466;
    let t34928 = 0.51123901271894332902e0_f64 * t34927;
    let t34930 = t2487 * t20843 * t3395;
    let t34931 = 0.59644551483876721719e0_f64 * t34930;
    let t34935 = 0.50050685932590597338e1_f64 * t1415 * t10546 * t1645 * t4807;
    let t34936 = t1436 * t10399;
    let t34937 = 0.51123901271894332902e0_f64 * t34936;
    let t34939 = 0.15889106645266856297e0_f64 * t18482 * t31540;
    (t34928, t34931, t34935, t34937, t34939)
}
