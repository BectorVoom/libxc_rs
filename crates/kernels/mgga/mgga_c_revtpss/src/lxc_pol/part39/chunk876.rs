//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 876/1348 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk876<F: Float>(t1774: F, t3759: F, t1280: F, t5245: F, t354: F, t471: F, t1214: F, t5351: F, t3766: F, t487: F, t460: F, t3302: F, t3603: F, t1248: F, t5332: F, t1269: F, t1287: F, t1794: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t5449 = t3759 * t1774;
    let t5452 = t1280 * t5245;
    let t5457 = t354 * t471;
    let t5458 = t5457 * t1214;
    let t5459 = t5351 * t5458;
    let t5462 = t3766 * t487;
    let t5463 = t460 * t5462;
    let t5464 = t3302 * t3603;
    let t5465 = t5464 * t1248;
    let t5466 = t5332 * t5465;
    let t5470 = t1269 * t1794 * t1287;
    (t5449, t5452, t5457, t5458, t5459, t5462, t5463, t5464, t5465, t5466, t5470)
}
