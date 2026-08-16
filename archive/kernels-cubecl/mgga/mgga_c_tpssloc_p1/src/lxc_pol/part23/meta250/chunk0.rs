//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 909/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk909<F: Float>(t3359: F, t6052: F, t11352: F, t6036: F, t1098: F, t5983: F, t11243: F, t5992: F, t11265: F, t1128: F, t6031: F, t1147: F, t6063: F) -> (F, F, F, F, F, F, F) {
    let t18643 = t6052 * t3359;
    let t18650 = t6036 * t11352;
    let t18686 = t5983 * t1098;
    let t18746 = t11243 * t5992;
    let t18754 = t11265 * t5992;
    let t18840 = t6031 * t1128;
    let t18899 = t6063 * t1147;
    (t18643, t18650, t18686, t18746, t18754, t18840, t18899)
}
