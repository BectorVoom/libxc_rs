//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 610/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk610<F: Float>(t2530: F, t2949: F, t1445: F, t813: F, t2197: F, t3492: F, t10713: F, t833: F, t10717: F, t3451: F, t590: F, t1022: F, t5241: F) -> (F, F, F, F, F, F) {
    let t11038 = t2949 * t2530;
    let t11039 = t1445 * t11038;
    let t11041 = F::new(0.46011511144704899612e1) * t813 * t11039;
    let t11043 = F::new(0.11502877786176224903e2) * t2197 * t3492;
    let t11044 = t1445 * t10713;
    let t11046 = F::new(0.11502877786176224903e2) * t833 * t11044;
    let t11047 = t1445 * t10717;
    let t11049 = F::new(0.11502877786176224903e2) * t833 * t11047;
    let t11050 = t3451 * t590;
    let t11053 = t5241 * t1022;
    (t11041, t11043, t11046, t11049, t11050, t11053)
}
