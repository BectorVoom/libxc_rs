//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 642/923 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk642<F: Float>(t13847: F, t2685: F, t2684: F, t969: F, t825: F, t12653: F, t12223: F, t935: F, t1445: F, t813: F, t12213: F) -> (F, F, F, F, F, F, F, F, F) {
    let t13848 = t2685 * t13847;
    let t13849 = t2684 * t13848;
    let t13851 = t969 * t13847;
    let t13852 = t825 * t13851;
    let t13855 = 0.38342925953920749677e0 * t12653;
    let t13857 = t12223 * t935;
    let t13858 = t1445 * t13857;
    let t13859 = t813 * t13858;
    let t13861 = t12213 * t935;
    (t13848, t13849, t13851, t13852, t13855, t13857, t13858, t13859, t13861)
}
