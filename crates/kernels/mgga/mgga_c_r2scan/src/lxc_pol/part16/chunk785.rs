//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 785/1112 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk785<F: Float>(t3034: F, t725: F, t41: F, t5812: F, t5815: F, t5818: F, t5821: F, t5925: F, t5936: F, t5940: F, t5945: F, t5950: F, t5959: F, t5963: F, t5834: F, t5966: F, t5968: F, t5970: F, t5972: F, t5975: F, t5976: F, t5978: F, t5980: F, t5982: F, t5985: F, t7849: F) -> (F, F) {
    let t9014 = t3034 * t725;
    let t9015 = t41 * t9014;
    let t9017 = t5812 + t5815 + t5925 - t9015 - t5818 + t5821 + 0.72290542002011598948e-2 * t5936 + t5940 + t5945 - t5950 + t5959 + t5963;
    let t9025 = -t5966 + 0.21687162600603479684e-1 * t5968 - 0.32106488758451047386e0 * t5970 - 0.1301229756036208781e0 * t5972 - t5975 + 8.0 * t5976 - 0.11290853155555555555e-2 * t5978 + t5834 + 8.0 * t5980 - 20.0 * t5982 + t5985 + t7849;
    (t9017, t9025)
}
