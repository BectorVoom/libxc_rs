//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 1662/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1662<F: Float>(t4353: F, t9794: F, t10760: F, t10890: F, t1549: F, t10811: F, t4462: F, t4416: F, t808: F, t10886: F, t2703: F, t4458: F) -> (F, F, F, F, F, F) {
    let t14760 = t9794 * t4353;
    let t14761 = t10760 * t14760;
    let t14765 = t10890 * t1549;
    let t14777 = t10811 * t4462;
    let t14779 = t808 * t4416;
    let t14780 = t10886 * t14779;
    let t14783 = F::new(7.0) / F::new(72.0) * t2703 * t4458;
    (t14761, t14765, t14777, t14779, t14780, t14783)
}
