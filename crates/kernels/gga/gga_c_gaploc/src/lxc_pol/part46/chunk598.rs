//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 598/1029 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk598<F: Float>(t3411: F, t4953: F, t8097: F, t874: F, t1445: F, t1562: F, t1641: F, t3391: F, t7980: F, t574: F, t2293: F, t2778: F) -> (F, F, F, F, F) {
    let t10363 = F::new(0.69017266717057349418e1) * t4953 * t3411;
    let t10364 = t8097 * t874;
    let t10365 = t1445 * t10364;
    let t10367 = F::new(0.69017266717057349418e1) * t1562 * t10365;
    let t10369 = F::new(0.46011511144704899612e1) * t1641 * t3391;
    let t10370 = t7980 * t874;
    let t10371 = t1445 * t10370;
    let t10373 = F::new(0.46011511144704899612e1) * t574 * t10371;
    let t10374 = t2778 * t2293;
    (t10363, t10367, t10369, t10373, t10374)
}
