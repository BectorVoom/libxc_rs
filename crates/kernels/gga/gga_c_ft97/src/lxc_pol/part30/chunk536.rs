//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 536/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk536<F: Float>(t444: F, t6240: F, t2691: F, t6247: F, t4113: F, t24357: F, t6256: F, t10363: F, t6: F, t8: F, t24330: F, t6249: F, t6250: F, t1472: F, t24287: F, t1476: F, t2360: F) -> (F, F, F, F, F, F, F, F) {
    let t25069 = t6240 * t444;
    let t25070 = t2691 * t25069;
    let t25076 = t6247 * t444;
    let t25077 = t4113 * t25076;
    let t25106 = t6256 * t24357;
    let t25110 = t10363 * t6;
    let t25111 = t25110 * t8;
    let t25112 = t4113 * t25111;
    let t25118 = t6249 * t24330 * t6250;
    let t25132 = 0.11113000182098765433e-1 * t1472 * t24287;
    let t25140 = t1476 * t2360;
    (t25069, t25070, t25077, t25106, t25112, t25118, t25132, t25140)
}
