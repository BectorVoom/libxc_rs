//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 610/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk610(t2530: f64, t2949: f64, t1445: f64, t813: f64, t2197: f64, t3492: f64, t10713: f64, t833: f64, t10717: f64, t3451: f64, t590: f64, t1022: f64, t5241: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t11038 = t2949 * t2530;
    let t11039 = t1445 * t11038;
    let t11041 = 0.46011511144704899612e1_f64 * t813 * t11039;
    let t11043 = 0.11502877786176224903e2_f64 * t2197 * t3492;
    let t11044 = t1445 * t10713;
    let t11046 = 0.11502877786176224903e2_f64 * t833 * t11044;
    let t11047 = t1445 * t10717;
    let t11049 = 0.11502877786176224903e2_f64 * t833 * t11047;
    let t11050 = t3451 * t590;
    let t11053 = t5241 * t1022;
    (t11041, t11043, t11046, t11049, t11050, t11053)
}
