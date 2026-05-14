//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1146/1296 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1146<F: Float>(t1445: F, t1562: F, t2293: F, t8097: F, t2854: F, t6393: F, t10448: F, t4953: F, t10374: F, t4614: F, t574: F, t3391: F, t4634: F, t10371: F, t1641: F, t10385: F, t1580: F) -> (F, F, F, F, F, F, F) {
    let t34032 = 0.13803453343411469884e2 * t1562 * t1445 * t8097 * t2293;
    let t34036 = 0.69017266717057349418e1 * t1562 * t1445 * t2854 * t6393;
    let t34038 = 0.13803453343411469884e2 * t4953 * t10448;
    let t34052 = 0.12269736305254639897e2 * t574 * t4614 * t10374;
    let t34054 = 0.46011511144704899612e1 * t4634 * t3391;
    let t34056 = 0.92023022289409799224e1 * t1641 * t10371;
    let t34058 = 0.23005755572352449806e2 * t1580 * t10385;
    (t34032, t34036, t34038, t34052, t34054, t34056, t34058)
}
