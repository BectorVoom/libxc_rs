//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1374/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1374<F: Float>(t1411: F, t19068: F, t32045: F, t32042: F, t33373: F, t20634: F, t3502: F, t32022: F, t33451: F, t1339: F, t32004: F, t33604: F, t109717: F, t33508: F, t2262: F, t33608: F, t3732: F) -> (F, F, F, F, F, F, F) {
    let t114260 = t1411 * t32045 * t19068;
    let t114264 = t33373 * t32042;
    let t114268 = t1411 * t32045 * t20634 * t3502;
    let t114271 = 0.18518518518518518519e-1 * t32022 * t33451;
    let t114273 = t1339 * t33604 * t32004;
    let t114276 = t1411 * t109717 * t33508;
    let t114280 = t1411 * t33608 * t2262 * t3732;
    (t114260, t114264, t114268, t114271, t114273, t114276, t114280)
}
