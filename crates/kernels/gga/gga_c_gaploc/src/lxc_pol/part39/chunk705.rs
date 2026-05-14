//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 705/1028 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk705<F: Float>(t13772: F, t13785: F, t13804: F, t13834: F, t502: F, t3749: F, t977: F, t1960: F, t2592: F, t123: F, t3720: F, t883: F, t2685: F, t2684: F, t969: F, t825: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t13836 = t13772 + t13785 + t13804 + t13834;
    let t13837 = t502 * t13836;
    let t13838 = t3749 * t977;
    let t13839 = t1960 * t13838;
    let t13841 = t2592 * t3749;
    let t13846 = t3720 * t123;
    let t13847 = t13846 * t883;
    let t13848 = t2685 * t13847;
    let t13849 = t2684 * t13848;
    let t13851 = t969 * t13847;
    let t13852 = t825 * t13851;
    (t13836, t13837, t13838, t13839, t13841, t13846, t13847, t13848, t13849, t13851, t13852)
}
