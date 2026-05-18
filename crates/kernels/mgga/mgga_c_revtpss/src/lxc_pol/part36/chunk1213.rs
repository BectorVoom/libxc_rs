//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1213/1378 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1213<F: Float>(t843: F, t1962: F, t41154: F, t25373: F, t25392: F, t25386: F, t25372: F, t11015: F, t7018: F, t25300: F, t9285: F, t25299: F) -> (F, F, F, F, F, F, F) {
    let t92612 = F::new(1232.0) / F::new(27.0) * t843;
    let t92742 = t1962 * t41154;
    let t92837 = t25373 * t25392;
    let t92838 = t25386 * t92837;
    let t92843 = t25372 * t92837;
    let t92861 = F::new(0.30356481678079769392e-1) * t7018 * t11015;
    let t92868 = t25300 * t9285;
    let t92870 = F::new(0.68540937416128198417e-2) * t25299 * t92868;
    (t92612, t92742, t92838, t92843, t92861, t92868, t92870)
}
