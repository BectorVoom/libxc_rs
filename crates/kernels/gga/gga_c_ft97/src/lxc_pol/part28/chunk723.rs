//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 723/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk723<F: Float>(t108: F, t7165: F, t379: F, t7824: F, t5498: F, t7162: F, t5493: F, t7150: F, t1322: F, t1774: F, t7151: F, t1308: F) -> (F, F, F, F, F, F, F) {
    let t32019 = t7165 * t108;
    let t32021 = t7824 * t32019 * t379;
    let t32025 = t7162 * t5498 / F::new(18.0);
    let t32026 = t5493 * t7150;
    let t32029 = t1774 * t1322;
    let t32031 = t7151 * t32029 / F::new(18.0);
    let t32032 = t1308 * t379;
    (t32019, t32021, t32025, t32026, t32029, t32031, t32032)
}
