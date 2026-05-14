//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1022/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1022<F: Float>(t1492: F, t8255: F, t486: F, t27191: F, t4231: F, t6368: F, t2259: F, t6351: F, t26987: F, t4204: F, t4203: F, t27010: F, t6369: F, t14321: F, t8241: F, t1286: F, t8077: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t27219 = t1492 * t8255;
    let t27220 = t486 * t27219;
    let t27222 = t4231 * t27191;
    let t27223 = t6368 * t27222;
    let t27225 = t2259 * t6351;
    let t27227 = t4204 * t26987;
    let t27228 = t4203 * t27227;
    let t27230 = t6369 * t27010;
    let t27231 = t6368 * t27230;
    let t27233 = t14321 * t8241;
    let t27235 = t8077 * t1286;
    (t27220, t27222, t27223, t27225, t27227, t27228, t27230, t27231, t27233, t27235)
}
