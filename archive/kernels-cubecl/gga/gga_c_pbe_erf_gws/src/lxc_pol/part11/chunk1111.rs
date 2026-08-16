//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1111/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1111<F: Float>(t1820: F, t1885: F, t31352: F, t3454: F, t40865: F, t40867: F, t47556: F, t7062: F, t7063: F, t40079: F, t954: F, t7115: F, t7759: F) -> (F, F, F, F, F, F) {
    let t47760 = F::cast_from(16.0_f64) / F::cast_from(5.0_f64) * t1820 * t1885 * t31352 * t3454;
    let t47761 = F::cast_from(16.0_f64) / F::cast_from(15.0_f64) * t40865;
    let t47762 = F::cast_from(32.0_f64) / F::cast_from(15.0_f64) * t40867;
    let t47765 = F::cast_from(32.0_f64) / F::cast_from(15.0_f64) * t7062 * t7063 * t47556;
    let t47766 = t40079 * t954;
    let t47769 = F::cast_from(16.0_f64) / F::cast_from(9.0_f64) * t7115 * t7759 * t47766;
    (t47760, t47761, t47762, t47765, t47766, t47769)
}
