//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 573/1028 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk573<F: Float>(t10351: F, t6716: F, t10314: F, t6711: F, t6710: F, t2859: F, t9333: F, t3410: F, t4614: F, t1562: F, t3411: F, t4953: F, t8097: F, t874: F, t1445: F, t1641: F, t3391: F) -> (F, F, F, F, F, F, F) {
    let t10353 = 0.69017266717057349418e1 * t6716 * t10351;
    let t10354 = t6711 * t10314;
    let t10356 = 0.11502877786176224903e2 * t6710 * t10354;
    let t10358 = 0.10725146985555128001e1 * t2859 * t9333;
    let t10359 = t4614 * t3410;
    let t10361 = 0.92023022289409799224e1 * t1562 * t10359;
    let t10363 = 0.69017266717057349418e1 * t4953 * t3411;
    let t10364 = t8097 * t874;
    let t10365 = t1445 * t10364;
    let t10367 = 0.69017266717057349418e1 * t1562 * t10365;
    let t10369 = 0.46011511144704899612e1 * t1641 * t3391;
    (t10353, t10356, t10358, t10361, t10363, t10367, t10369)
}
