//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 751/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk751<F: Float>(t5015: F, t386: F, t518: F, t85: F, t462: F) -> (F, F, F, F) {
    let t5016 = 0.73245789224026180216e-3 * t5015;
    let t5018 = t386 * t518 * t85;
    let t5019 = t462 * t5018;
    let t5020 = 0.56968947174242584612e-3 * t5019;
    (t5016, t5018, t5019, t5020)
}
