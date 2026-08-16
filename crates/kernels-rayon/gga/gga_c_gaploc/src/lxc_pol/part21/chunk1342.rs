//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1342/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1342(t34930: f64, t10546: f64, t1415: f64, t1645: f64, t4807: f64, t10399: f64, t1436: f64, t18482: f64, t31540: f64, t10470: f64, t4849: f64, t10430: f64, t587: f64, t589: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t34931 = 0.59644551483876721719e0_f64 * t34930;
    let t34935 = 0.50050685932590597338e1_f64 * t1415 * t10546 * t1645 * t4807;
    let t34936 = t1436 * t10399;
    let t34937 = 0.51123901271894332902e0_f64 * t34936;
    let t34939 = 0.15889106645266856297e0_f64 * t18482 * t31540;
    let t34941 = 0.51123901271894332902e1_f64 * t4849 * t10470;
    let t34943 = t587 * t589 * t10430;
    (t34931, t34935, t34937, t34939, t34941, t34943)
}
