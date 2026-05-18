//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 1118/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk1118<F: Float>(t41136: F, t43619: F, t43627: F, t43630: F, t43636: F, t43640: F, t43642: F, t43645: F, t43647: F, t43648: F, t43650: F, t43653: F) -> F {
    let t47280 = F::new(0.15337170381568299871e1) * t41136;
    let t47281 = t43619 + t43627 + t43630 + t43636 + t43640 + F::new(0.11502877786176224903e2) * t43642 + t43645 + t43647 - t43648 + F::new(0.9585731488480187419e0) * t43650 + t43653 - t47280;
    t47281
}
