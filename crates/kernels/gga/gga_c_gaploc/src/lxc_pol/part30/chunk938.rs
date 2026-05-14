//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 938/1268 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk938<F: Float>(t2530: F, t2949: F, t1445: F, t813: F, t2197: F, t3492: F, t10713: F, t833: F, t10717: F, t3451: F, t590: F, t1022: F, t5241: F, t2679: F, t9805: F, t1029: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t11038 = t2949 * t2530;
    let t11039 = t1445 * t11038;
    let t11041 = 0.46011511144704899612e1 * t813 * t11039;
    let t11043 = 0.11502877786176224903e2 * t2197 * t3492;
    let t11044 = t1445 * t10713;
    let t11046 = 0.11502877786176224903e2 * t833 * t11044;
    let t11047 = t1445 * t10717;
    let t11049 = 0.11502877786176224903e2 * t833 * t11047;
    let t11050 = t3451 * t590;
    let t11053 = t5241 * t1022;
    let t11054 = t11053 * t2679;
    let t11055 = t9805 * t11054;
    let t11056 = 0.57514388930881124514e0 * t11055;
    let t11057 = t1029 * t2679;
    (t11038, t11039, t11041, t11043, t11044, t11046, t11047, t11049, t11050, t11053, t11054, t11056, t11057)
}
