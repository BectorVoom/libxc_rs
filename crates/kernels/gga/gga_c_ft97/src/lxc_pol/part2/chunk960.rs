//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 960/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk960<F: Float>(t14685: F, t14904: F, t788: F, t2648: F, t4134: F, t10235: F, t1217: F, t14635: F, t14637: F, t14639: F, t14619: F, t14622: F, t14626: F, t14630: F, t14633: F, t14642: F, t14645: F, t14650: F) -> (F, F, F, F) {
    let t14905 = t14685 + t14904;
    let t14906 = t788 * t14905;
    let t14911 = t2648 * t4134;
    let t14914 = t10235 * t1217;
    let t14921 = F::new(2.0) / F::new(9.0) * t14635;
    let t14922 = F::new(4.0) / F::new(9.0) * t14637;
    let t14923 = F::new(4.0) / F::new(27.0) * t14639;
    let t14927 = -F::new(8.0) / F::new(3.0) * t14619 + F::new(8.0) / F::new(9.0) * t14622 + t14626 / F::new(3.0) - F::new(4.0) / F::new(3.0) * t14630 + F::new(2.0) / F::new(3.0) * t14633 - t14921 - t14922 + t14923 - F::new(2.0) / F::new(9.0) * t14642 - F::new(10.0) / F::new(27.0) * t14645 - F::new(2.0) / F::new(3.0) * t14650;
    (t14906, t14911, t14914, t14927)
}
