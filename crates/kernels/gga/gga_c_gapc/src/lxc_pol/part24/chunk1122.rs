//! GGA_C_GAPC lxc pol — lxc_pol part 24 (v4rho2sigma2_3) CSE chunk 1122/1327 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part24_v4rho2sigma2_3_chunk1122<F: Float>(t1089: F, t3687: F, t9906: F, t11945: F, t9895: F, t11878: F, t15805: F, t1936: F, t3775: F, t9980: F, t10079: F, t11430: F, t3363: F) -> (F, F, F, F, F) {
    let t33850 = t9906 * t3687 * t1089;
    let t33852 = t9895 * t11945;
    let t33855 = t15805 * t1936 * t11878;
    let t33857 = t3775 * t9980;
    let t33863 = t3363 * t11430 * t10079;
    (t33850, t33852, t33855, t33857, t33863)
}
