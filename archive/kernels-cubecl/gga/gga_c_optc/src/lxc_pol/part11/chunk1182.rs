//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1182/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1182<F: Float>(t4297: F, t53825: F, t11885: F, t18183: F, t4281: F, t15082: F, t5249: F, t17619: F, t4215: F, t17360: F, t241: F, t1220: F, t17464: F, t2367: F) -> (F, F, F, F, F, F) {
    let t53826 = t4297 * t53825;
    let t53829 = t4281 * t11885 * t18183;
    let t53831 = t5249 * t15082;
    let t53851 = t17619 * t4215;
    let t53885 = t241 * t17360;
    let t53909 = t1220 * t2367 * t17464;
    (t53826, t53829, t53831, t53851, t53885, t53909)
}
