//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1421/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1421<F: Float>(t35099: F, t10241: F, t20550: F, t15482: F, t20549: F, t1: F, t31740: F, t544: F, t10540: F, t18067: F, t2365: F, t25730: F, t4391: F) -> (F, F, F, F, F) {
    let t35100 = F::new(0.2556195063594716645e0) * t35099;
    let t35101 = t20550 * t10241;
    let t35104 = F::new(0.34082600847929555269e0) * t20549 * t15482 * t35101;
    let t35106 = t544 * t31740 * t1;
    let t35109 = t18067 * t10540;
    let t35110 = F::new(0.59584149919750711116e-1) * t35109;
    let t35112 = t4391 * t2365 * t25730;
    (t35100, t35104, t35106, t35110, t35112)
}
