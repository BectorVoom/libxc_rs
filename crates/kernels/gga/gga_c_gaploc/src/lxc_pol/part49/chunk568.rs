//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 568/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk568<F: Float>(t3225: F, t716: F, t9664: F, t9666: F, t9669: F, t9672: F, t9674: F, t2524: F, t871: F, t3228: F, t471: F, t64: F) -> (F, F, F, F) {
    let t9676 = t3225 * t716;
    let t9678 = -F::new(21.0) / F::new(512.0) * t9664 + F::new(147.0) / F::new(16384.0) * t9666 - F::new(63.0) / F::new(1048576.0) * t9669 + F::new(21.0) / F::new(1048576.0) * t9672 - F::new(49.0) / F::new(16384.0) * t9674 + F::new(7.0) / F::new(512.0) * t9676;
    let t9682 = t2524 * t871;
    let t9688 = t9678 * t471 - F::new(4.0) / F::new(3.0) * t3228 * t64 + t9682 / F::new(2.0) - F::new(7.0) / F::new(512.0) * t9664 + F::new(21.0) / F::new(16384.0) * t9666 - F::new(7.0) / F::new(16384.0) * t9674 + F::new(7.0) / F::new(1536.0) * t9676;
    (t9676, t9678, t9682, t9688)
}
