//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 705/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk705<F: Float>(t1520: F, t6241: F, t2282: F, t4165: F, t4170: F, t196: F, t5798: F, t1173: F, t476: F, t458: F, t1429: F, t1434: F, t1460: F, t2221: F, t2225: F, t2242: F, t4253: F, t460: F, t5928: F, t5933: F, t5937: F, t5949: F, t5954: F, t5958: F) -> (F, F, F, F, F, F, F, F) {
    let t6242 = t6241 * t1520;
    let t6243 = t4165 * t2282;
    let t6244 = t2282 * t1520;
    let t6246 = 2.0 * t4170 * t6244;
    let t6247 = t5798 * t196;
    let t6256 = t476 * t1173;
    let t6267 = t476 * t458;
    let t6272 = 0.619125e-2 * t6247 * t460 + 0.9286875e-2 * t2242 * t1429 - 0.619125e-2 * t2242 * t1434 + 0.9286875e-2 * t1460 * t2221 + 0.46434375e-2 * t6256 * t5928 - 0.9286875e-2 * t4253 * t5933 + 0.9286875e-2 * t476 * t5937 - 0.619125e-2 * t1460 * t2225 - 0.9286875e-2 * t4253 * t5949 + 0.123825e-1 * t6267 * t5954 - 0.619125e-2 * t476 * t5958;
    (t6242, t6243, t6244, t6246, t6247, t6256, t6267, t6272)
}
