//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1811/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1811<F: Float>(t25040: F, t82074: F, t87712: F, t82294: F, t25193: F, t81591: F, t28: F, t40772: F, t1649: F, t2752: F, t1437: F, t6509: F) -> (F, F, F, F, F, F) {
    let t87927 = t87712 * t82074 * t25040;
    let t87929 = F::cast_from(0.10417915756705434098e0_f64) * t82294;
    let t87931 = t81591 * t25193;
    let t89953 = t40772 * t28;
    let t89992 = t2752 * t1649;
    let t90090 = t6509 * t1437;
    (t87927, t87929, t87931, t89953, t89992, t90090)
}
