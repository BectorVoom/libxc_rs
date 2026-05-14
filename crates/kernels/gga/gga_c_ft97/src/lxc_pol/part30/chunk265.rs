//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 265/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk265<F: Float>(t1127: F, t2426: F, t709: F, t1103: F, t172: F, t228: F, t231: F, t227: F, t9: F, t706: F, t1123: F, t173: F, t701: F, t2440: F, t420: F, t3691: F) -> (F, F, F, F, F) {
    let t3790 = t2426 * t1127;
    let t3791 = t3790 * t709;
    let t3794 = t1103 * t172;
    let t3796 = t228 * t3794 * t231;
    let t3799 = t9 * t227 * t1103;
    let t3800 = t3799 * t706;
    let t3803 = t173 * t1123;
    let t3804 = t701 * t3803;
    let t3806 = t420 * t2440;
    let t3807 = t3806 * t3691;
    (t3791, t3796, t3800, t3804, t3807)
}
