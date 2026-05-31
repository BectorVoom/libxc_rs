//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1035/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1035<F: Float>(t1508: F, t1571: F, t4416: F, t4779: F, t584: F, t1461: F, t1561: F, t1397: F, t4390: F, t1238: F, t4072: F, t4081: F, t92: F) -> (F, F, F, F, F, F) {
    let t17551 = t1508 * t1571;
    let t17568 = t584 * t4779 * t4416;
    let t17571 = t1461 * t1561;
    let t18067 = t1397 * t4390;
    let t18089 = F::cast_from(1.0_f64) / t4072 / t1238;
    let t18091 = t18089 * t92 * t4081;
    (t17551, t17568, t17571, t18067, t18089, t18091)
}
