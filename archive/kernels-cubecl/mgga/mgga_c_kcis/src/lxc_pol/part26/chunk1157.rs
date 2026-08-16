//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1157/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1157<F: Float>(t27520: F, t29433: F, t20961: F, t585: F, t1468: F, t7296: F, t27544: F, t7299: F, t2055: F, t5748: F, t2062: F, t5752: F) -> (F, F, F, F, F, F) {
    let t29434 = t27520 * t29433;
    let t29436 = t20961 * t585;
    let t29438 = t1468 * t7296;
    let t29440 = t27544 * t7299;
    let t29442 = t5748 * t2055;
    let t29444 = t5752 * t2062;
    (t29434, t29436, t29438, t29440, t29442, t29444)
}
