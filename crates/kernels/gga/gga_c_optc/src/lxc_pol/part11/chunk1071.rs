//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1071/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1071<F: Float>(t15063: F, t17622: F, t17627: F, t43671: F, t11782: F, t18213: F, t4297: F, t11885: F, t18183: F, t4281: F, t15082: F, t5249: F, t17619: F, t4215: F, t17360: F, t241: F) -> (F, F, F, F, F, F, F) {
    let t53812 = t17622 * t15063;
    let t53823 = t43671 * t17627;
    let t53825 = t11782 * t18213;
    let t53826 = t4297 * t53825;
    let t53829 = t4281 * t11885 * t18183;
    let t53831 = t5249 * t15082;
    let t53851 = t17619 * t4215;
    let t53885 = t241 * t17360;
    (t53812, t53823, t53826, t53829, t53831, t53851, t53885)
}
