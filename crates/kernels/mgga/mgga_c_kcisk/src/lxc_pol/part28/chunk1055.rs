//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1055/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1055<F: Float>(t2532: F, t733: F, t7304: F, t23304: F, t7303: F, t7302: F, t1894: F, t8946: F, t5322: F, t7429: F, t1931: F, t9036: F, t17816: F, t2580: F, t1757: F, t5289: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t24174 = t733 * t2532;
    let t24175 = t24174 * t7304;
    let t24177 = t7303 * t23304;
    let t24178 = t7302 * t24177;
    let t24181 = t8946 * t1894;
    let t24182 = t5322 * t24181;
    let t24183 = t7429 * t24182;
    let t24185 = t1931 * t9036;
    let t24187 = t17816 * t2580;
    let t24189 = t8946 * t1757;
    let t24190 = t7303 * t24189;
    let t24191 = t5289 * t24190;
    (t24175, t24177, t24178, t24181, t24182, t24183, t24185, t24187, t24189, t24191)
}
