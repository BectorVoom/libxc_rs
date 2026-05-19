//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 914/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk914<F: Float>(t2009: F, t2021: F, t45524: F, t13592: F, t2033: F, t549: F, t2631: F, t36515: F, t787: F, t10827: F, t11053: F, t9805: F) -> (F, F, F, F) {
    let t45527 = F::cast_from(0.35750489951850426669e0_f64) * t2021 * t45524 * t2009;
    let t45529 = t2033 * t549 * t13592;
    let t45530 = F::cast_from(0.29792074959875355558e-1_f64) * t45529;
    let t45536 = F::cast_from(0.17875244975925213335e2_f64) * t787 * t36515 * t2631;
    let t45542 = t9805 * t11053 * t10827;
    (t45527, t45530, t45536, t45542)
}
