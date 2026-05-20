//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1342/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1342<F: Float>(t40135: F, t760: F, t10565: F, t606: F, t706: F, t10468: F, t750: F, t10555: F, t10605: F, t10436: F, t2398: F, t10356: F, t10439: F) -> (F, F, F, F, F, F) {
    let t40137 = F::cast_from(0.6233709278045326953e3_f64) * t760 * t40135;
    let t40139 = t706 * t10565 * t606;
    let t40140 = F::new(16.0) * t40139;
    let t40141 = t10468 * t750;
    let t40142 = F::new(4.0) * t40141;
    let t40143 = t10605 * t10555;
    let t40144 = F::new(144.0) * t40143;
    let t40145 = t2398 * t10436;
    let t40146 = F::new(48.0) * t40145;
    let t40148 = t10439 * t750 * t10356;
    (t40137, t40140, t40142, t40144, t40146, t40148)
}
