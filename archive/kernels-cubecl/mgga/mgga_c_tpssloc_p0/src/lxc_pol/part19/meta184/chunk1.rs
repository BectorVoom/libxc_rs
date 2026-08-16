//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 836/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk836<F: Float>(t2244: F, t751: F, t2658: F, t9853: F, t9859: F, t9911: F, t9914: F, t9917: F, t9921: F, t9923: F, t9925: F, t9928: F, t9931: F) -> (F, F, F) {
    let t9932 = t751 * t2244;
    let t9933 = t2658 * t9932;
    let t9934 = F::cast_from(36.0_f64) * t9933;
    let t9935 = t9853 + t9911 + t9914 + t9917 - t9921 - t9923 + t9925 + t9859 + t9928 + t9931 + t9934;
    (t9932, t9934, t9935)
}
