//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 587/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk587<F: Float>(t13940: F, t15109: F, t2367: F, t36: F, t2079: F, t262: F, t14290: F, t556: F, t14293: F, t2842: F, t27: F, t29: F, t570: F) -> (F, F, F, F, F, F) {
    let t15122 = t13940 * t15109;
    let t15128 = t36 * t2367;
    let t15130 = t2079 * t262 * t15128;
    let t15132 = t14290 * t556;
    let t15134 = t14293 * t2842;
    let t15137 = t27 * t29 * t570;
    (t15122, t15128, t15130, t15132, t15134, t15137)
}
