//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1053/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1053<F: Float>(t1785: F, t7623: F, t3670: F, t2133: F, t816: F, t1224: F, t65: F, t3698: F, t1234: F, t8184: F, t5362: F, t7613: F, t1256: F, t8177: F, t8185: F, t2137: F, t5389: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t29037 = t1785 * t7623;
    let t29040 = t3670 * t7623;
    let t29047 = t2133 * t816;
    let t29048 = t65 * t1224;
    let t29054 = t65 * t3698;
    let t29062 = t1234 * t8184;
    let t29065 = t7613 * t5362;
    let t29072 = t8177 * t1256;
    let t29077 = t8185 * t1256;
    let t29082 = t2137 * t5389;
    (t29037, t29040, t29047, t29048, t29054, t29062, t29065, t29072, t29077, t29082)
}
