//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1052/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1052<F: Float>(t19796: F, t4231: F, t4230: F, t19801: F, t6332: F, t6331: F, t1445: F, t485: F, t6318: F, t2271: F, t4193: F, t19928: F, t4185: F, t19895: F, t6317: F, t4203: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t21074 = t4231 * t19796;
    let t21075 = t4230 * t21074;
    let t21077 = t6332 * t19801;
    let t21078 = t6331 * t21077;
    let t21080 = t485 * t1445;
    let t21081 = t21080 * t6318;
    let t21083 = t2271 * t4193;
    let t21085 = t4231 * t19928;
    let t21086 = t4230 * t21085;
    let t21088 = t2271 * t4185;
    let t21090 = t6317 * t19895;
    let t21091 = t4203 * t21090;
    (t21074, t21075, t21077, t21078, t21081, t21083, t21085, t21086, t21088, t21090, t21091)
}
