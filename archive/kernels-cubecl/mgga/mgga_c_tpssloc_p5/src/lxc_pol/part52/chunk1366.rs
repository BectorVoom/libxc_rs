//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1366/1400 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1366<F: Float>(t652: F, t6534: F, t8103: F, t26168: F, t8690: F, t119799: F, t2114: F, t2165: F, t24932: F, t24980: F, t25958: F, t25965: F, t26098: F, t27863: F, t27888: F, t31880: F, t4077: F, t6539: F, t7264: F, t7266: F, t7408: F, t7451: F, t7472: F, t7670: F) -> F {
    let t122897 = t652 * t8103 * t6534;
    let t122910 = t8690 * t26168;
    let t122912 = -t2114 * t25958 - t2165 * t26098 - F::cast_from(2.0_f64) * t24932 * t7472 - F::cast_from(2.0_f64) * t24980 * t7266 - F::cast_from(2.0_f64) * t25965 * t7266 - F::cast_from(2.0_f64) * t27863 * t6539 - F::cast_from(2.0_f64) * t27888 * t7472 - F::cast_from(2.0_f64) * t31880 * t4077 - t7264 * t7670 - t7408 * t7451 - F::cast_from(3.0_f64) * t119799 - F::cast_from(2.0_f64) * t122897 + F::cast_from(3.0_f64) * t122910;
    t122912
}
