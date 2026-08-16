//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 1828/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1828<F: Float>(t1022: F, t883: F, t607: F, t14211: F, t3071: F, t1615: F, t360: F, t4342: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t14212 = t1022 * t883;
    let t14213 = t14212 * t607;
    let t14214 = t14211 * t14213;
    let t14215 = t3071 * t14214;
    let t14218 = t1615 * t1022;
    let t14219 = t360 * t883;
    let t14220 = t14219 * t607;
    let t14221 = t14218 * t14220;
    let t14222 = t3071 * t14221;
    let t14227 = t607 * t1022;
    let t14228 = t14227 * t360;
    let t14229 = t4342 * t14228;
    let t14230 = t3071 * t14229;
    (t14212, t14213, t14214, t14215, t14218, t14219, t14220, t14221, t14222, t14228, t14229, t14230)
}
