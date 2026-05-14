//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1019/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1019<F: Float>(t2279: F, t6344: F, t26411: F, t6322: F, t6321: F, t25296: F, t4231: F, t6368: F, t1501: F, t8279: F, t1413: F, t8231: F, t1489: F, t6377: F, t6388: F, t1493: F, t8233: F, sigma0: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t27170 = t6344 * t2279;
    let t27172 = t6322 * t26411;
    let t27173 = t6321 * t27172;
    let t27175 = t4231 * t25296;
    let t27176 = t6368 * t27175;
    let t27178 = t1501 * t8279;
    let t27180 = t8231 * t1413;
    let t27181 = t27180 * sigma0;
    let t27182 = t27181 * t1489;
    let t27184 = t6388 * t6377;
    let t27186 = t8233 * t1493;
    (t27170, t27172, t27173, t27175, t27176, t27178, t27180, t27182, t27184, t27186)
}
