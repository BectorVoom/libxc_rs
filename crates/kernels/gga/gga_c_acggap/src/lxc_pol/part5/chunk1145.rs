//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1145/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1145<F: Float>(t1017: F, t1150: F, t12753: F, t12755: F, t1314: F, t15574: F, t15576: F, t18129: F, t20400: F, t20519: F, t20545: F, t335: F, t3616: F, t367: F, t4487: F, t4593: F, t5170: F, t6308: F, t6387: F, t922: F, t960: F) -> F {
    let t20550 = -F::new(0.16006300097412701803e-1) * t12753 + F::new(0.34299214494455789578e-2) * t20519 - F::new(0.25724410870841842183e-2) * t15574 - F::new(0.45351183609335988442e-1) * t15576 - F::new(0.85748036236139473944e-3) * t12755 + t1150 * t960 * t6308 * t922 / F::new(8.0) - t3616 * t960 * t6387 * t922 / F::new(4.0) + t1150 * t18129 * t1314 / F::new(4.0) - t3616 * t4593 * t4487 / F::new(2.0) - t367 * t4593 * t5170 / F::new(8.0) - t335 * t960 * t20400 * t1017 / F::new(24.0) + t367 * t960 * t20545 * t1017 / F::new(4.0);
    t20550
}
