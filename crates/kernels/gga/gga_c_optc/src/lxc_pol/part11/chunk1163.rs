//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1163/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1163<F: Float>(t1220: F, t17439: F, t2367: F, t487: F, t8287: F, t17534: F, t17539: F, t870: F, t20: F, t4298: F, t496: F, t5: F) -> (F, F, F, F) {
    let t52324 = t1220 * t2367 * t17439;
    let t52326 = t8287 * t487;
    let t52329 = t52326 * t870 * t17534 * t17539;
    let t52330 = t4298 * t20;
    let t52331 = t5 * t496;
    (t52324, t52329, t52330, t52331)
}
