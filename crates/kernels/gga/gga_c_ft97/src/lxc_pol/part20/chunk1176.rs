//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1176/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1176<F: Float>(t25462: F, t28947: F, t28960: F, t6210: F, t1466: F, t29016: F, t681: F, t7021: F, t880: F, t193: F, t2405: F, t2413: F, t25412: F, t2665: F, t28950: F, t28955: F, t28966: F, t28985: F, t29033: F, t4309: F, t6216: F, t6222: F, t684: F, t6970: F, t824: F, t98257: F, t98268: F, t98694: F) -> (F,) {
    let t111657 = 2.0 / 81.0 * t25462 * t28947;
    let t111664 = t6210 * t28960 / 9.0;
    let t111667 = t1466 * t681 * t29016 / 9.0;
    let t111668 = t7021 * t880;
    let t111679 = 2.0 / 9.0 * t6216 * t98694 * t28950 + 2.0 / 9.0 * t6216 * t25412 * t29033 * t684 + 2.0 / 9.0 * t6216 * t25412 * t28966 * t684 + t6216 * t25412 * t6970 * t2413 / 9.0 + 2.0 / 27.0 * t6216 * t98268 * t6970 * t2405 + t111657 - t98257 - 2.0 / 3.0 * t1466 * t193 * t6222 * t4309 * t824 - t111664 - t111667 - t6216 * t2665 * t111668 * t684 / 9.0 - t6216 * t2665 * t28985 * t2413 / 18.0 + t6210 * t28955 / 3.0;
    (t111679,)
}
