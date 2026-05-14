//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1327/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1327<F: Float>(t1065: F, t32683: F, t3299: F, t9406: F, t32684: F, t32553: F, t32582: F, t32588: F, t32556: F, t33325: F, t33328: F, t33330: F, t109152: F, t109154: F, t109160: F, t109162: F, t109165: F, t110815: F, t1624: F, t22160: F, t2709: F, t296: F, t32687: F, t33331: F, t33342: F, t3465: F, t5585: F, t9408: F, t9783: F, t9896: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t111533 = t1065 * t32683;
    let t111564 = t3299 * t9406;
    let t111577 = 3.0 * t32684;
    let t111582 = 6.0 * t32553;
    let t111583 = 18.0 * t32582;
    let t111584 = 3.0 * t32588;
    let t111585 = 6.0 * t32556;
    let t113271 = t33325 / 8.0;
    let t113272 = t33328 / 8.0;
    let t113273 = 2.0 * t33330;
    let t113288 = t9408 * t33342 / 8.0 - t32687 * t9783 / 8.0 + t109152 - t2709 * t5585 * t1624 / 8.0 - t109154 - t109160 + t109162 - t2709 * t296 * t22160 / 16.0 - t109165 + t9408 * t33331 / 8.0 + t110815 - t3465 * t9896 / 8.0;
    (t111533, t111564, t111577, t111582, t111583, t111584, t111585, t113271, t113272, t113273, t113288)
}
