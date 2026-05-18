//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 940/1088 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk940<F: Float>(t109: F, t24890: F, t490: F, t1001: F, t236: F, t3351: F, t618: F, t1970: F, t1971: F, t333: F, t511: F, t5605: F) -> (F, F) {
    let t40167 = t24890 * t109;
    let t40168 = t490 * t40167;
    let t40172 = t3351 * t40168 * t236 * t618 * t1001;
    let t40177 = t1970 * t1971 * t511 * t5605 * t333;
    (t40172, t40177)
}
