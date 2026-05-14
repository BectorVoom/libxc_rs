//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 771/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk771<F: Float>(t24526: F, t505: F, t2354: F, t446: F, t1424: F, t2360: F, t2349: F, t1434: F, t1435: F, t2399: F, t1934: F, t6135: F, t1433: F, t458: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t24527 = t24526 * t505;
    let t24528 = t2354 * t24527;
    let t24529 = t446 * t24528;
    let t24531 = t1424 * t2360;
    let t24532 = t24531 * t2349;
    let t24533 = t2354 * t24532;
    let t24534 = t446 * t24533;
    let t24537 = t1434 * t2399 * t1435;
    let t24538 = 2.0 / 9.0 * t24537;
    let t24539 = t6135 * t1934;
    let t24540 = t2354 * t24539;
    let t24541 = t446 * t24540;
    let t24543 = t1433 * t458;
    (t24528, t24529, t24531, t24533, t24534, t24537, t24538, t24540, t24541, t24543)
}
