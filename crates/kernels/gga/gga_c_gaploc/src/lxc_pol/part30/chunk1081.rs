//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1081/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1081<F: Float>(t107: F, t25760: F, t544: F, t4360: F, t8410: F, t1359: F, t2754: F, t4149: F, t986: F, t1397: F, t8330: F, t1415: F, t8265: F) -> (F, F, F, F, F, F) {
    let t26455 = t544 * t25760 * t107;
    let t26609 = t4360 * t8410;
    let t26629 = t1359 * t2754;
    let t26673 = t4149 * t986;
    let t26726 = t1397 * t8330;
    let t26763 = t1415 * t8265;
    (t26455, t26609, t26629, t26673, t26726, t26763)
}
