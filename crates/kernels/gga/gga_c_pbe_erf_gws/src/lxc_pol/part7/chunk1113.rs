//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1113/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1113<F: Float>(t2246: F, t4438: F, t2417: F, t8734: F, t829: F, t830: F, t19608: F, t19659: F, t19744: F, t19756: F, t19907: F, t19914: F, t19923: F, t2306: F, t2384: F, t2391: F, t2392: F, t2408: F, t2409: F, t3074: F, t4390: F, t4419: F, t6106: F, t6135: F, t6755: F, t6781: F, t6784: F, t6789: F, t6822: F, t827: F, t833: F, t8606: F, t9283: F) -> F {
    let t19925 = t2246 * t4438;
    let t19937 = t8734 * t2417;
    let t19939 = t829 * t830 * t19937;
    let t19950 = -F::new(7.0) / F::new(6.0) * t19907 + t19659 * t4390 / F::new(6.0) - t19608 * t19914 / F::new(6.0) + t2392 * t4419 / F::new(16.0) + t6106 * t2391 * t833 / F::new(32.0) - F::new(7.0) / F::new(72.0) * t19923 + F::new(7.0) / F::new(36.0) * t19925 - t2384 * t6135 / F::new(4.0) - t2384 * t6789 / F::new(8.0) - t2384 * t6784 / F::new(8.0) + t3074 * t2306 * t19744 * t8606 / F::new(8.0) + t827 * t19939 / F::new(4.0) + t2408 * t2409 * t6781 * t6755 / F::new(4.0) - t2408 * t9283 * t19756 * t6822 / F::new(2.0);
    t19950
}
