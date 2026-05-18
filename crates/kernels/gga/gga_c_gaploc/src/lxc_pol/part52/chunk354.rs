//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 354/1013 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk354<F: Float>(t1035: F, t702: F, t1024: F, t779: F, t2513: F, t2515: F, t2520: F, t2522: F, t1020: F, t471: F, t64: F) -> (F, F, F, F) {
    let t2909 = t1035 * t702;
    let t2912 = t779 * t1024;
    let t2919 = -F::new(21.0) / F::new(128.0) * t2513 + F::new(21.0) / F::new(4096.0) * t2515 - F::new(7.0) / F::new(4096.0) * t2520 + F::new(7.0) / F::new(128.0) * t2522;
    let t2925 = t2919 * t471 - F::new(4.0) / F::new(3.0) * t1020 * t64 - F::new(7.0) / F::new(128.0) * t2513 + F::new(7.0) / F::new(384.0) * t2522;
    (t2909, t2912, t2919, t2925)
}
