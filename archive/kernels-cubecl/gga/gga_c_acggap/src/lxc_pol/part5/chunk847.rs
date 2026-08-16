//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 847/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk847<F: Float>(t813: F, t224: F, t2827: F, t2627: F, t883: F, t273: F, t2787: F, t286: F, t791: F, t709: F, t804: F, t36: F, t7777: F, t88: F) -> (F, F, F, F, F, F) {
    let t11882 = t813 * t813;
    let t11883 = F::cast_from(1.0_f64) / t11882;
    let t11889 = t224 * t2827;
    let t11893 = t883 * t2627;
    let t11898 = F::cast_from(0.46785788981077169656e1_f64) * t286 * t791 * t2787 * t273;
    let t11900 = F::cast_from(120.0_f64) * t709 * t804;
    let t11906 = F::cast_from(840.0_f64) * t36 * t7777 * t88;
    (t11883, t11889, t11893, t11898, t11900, t11906)
}
