//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 716/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk716<F: Float>(t2592: F, t3749: F, t123: F, t3720: F, t883: F, t2685: F, t2684: F, t969: F, t825: F, t12653: F, t12223: F, t935: F) -> (F, F, F, F, F, F, F, F, F) {
    let t13841 = t2592 * t3749;
    let t13846 = t3720 * t123;
    let t13847 = t13846 * t883;
    let t13848 = t2685 * t13847;
    let t13849 = t2684 * t13848;
    let t13851 = t969 * t13847;
    let t13852 = t825 * t13851;
    let t13855 = F::new(0.38342925953920749677e0) * t12653;
    let t13857 = t12223 * t935;
    (t13841, t13846, t13847, t13848, t13849, t13851, t13852, t13855, t13857)
}
