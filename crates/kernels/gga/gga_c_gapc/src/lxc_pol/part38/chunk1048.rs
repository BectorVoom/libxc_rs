//! GGA_C_GAPC lxc pol — lxc_pol part 38 (v4rho2sigma2_17) CSE chunk 1048/1307 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part38_v4rho2sigma2_17_chunk1048<F: Float>(t7556: F, t966: F, t9864: F, t314: F, t6: F, t959: F, t1038: F, t19159: F, t3787: F, t2546: F, t286: F, t2553: F, t3074: F, t4: F, t8133: F) -> (F, F, F, F, F) {
    let t29228 = t7556 * t966 * t9864;
    let t29314 = t6 * t959 * t314;
    let t29350 = t3787 * t1038 * t19159;
    let t29435 = t2546 * t286;
    let t29473 = t2553 * t3074 * t8133 * t4;
    (t29228, t29314, t29350, t29435, t29473)
}
