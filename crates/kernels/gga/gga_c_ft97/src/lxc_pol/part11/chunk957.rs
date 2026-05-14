//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 957/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk957<F: Float>(t761: F, t9570: F, t766: F, t9571: F, t1882: F, t9989: F, t10059: F, t10004: F, t2576: F, t8232: F, t241: F, t41752: F, t10020: F, t9840: F, t10131: F, t10002: F, t10024: F, t10029: F, t10034: F, t2459: F, t2469: F, t2568: F, t2569: F, t2574: F, t2594: F, t265: F, t41753: F, t41794: F, t446: F, t729: F, t773: F, t9572: F, t9578: F) -> (F, F, F, F, F, F) {
    let t42416 = t761 * t9570;
    let t42417 = t9571 * t766;
    let t42422 = t1882 * t9989;
    let t42424 = t1882 * t10059;
    let t42430 = t1882 * t10004;
    let t42455 = t8232 * t2576;
    let t42469 = t41752 * t241;
    let t42474 = t1882 * t10020;
    let t42476 = t1882 * t9840;
    let t42482 = t1882 * t10131;
    let t42488 = -4.0 * t446 * t729 * t2568 * t2569 * t2459 + 16.0 / 9.0 * t42455 - 8.0 * t446 * t2574 * t2469 * t10029 - 8.0 * t446 * t729 * t10002 * t10034 - 40.0 / 81.0 * t446 * t10024 * t773 * t9572 - 80.0 / 243.0 * t446 * t42469 * t265 * t41753 - 4.0 / 3.0 * t42474 - 8.0 / 3.0 * t42476 + 16.0 / 9.0 * t446 * t2594 * t773 * t9578 + 4.0 / 27.0 * t42482 - t446 * t729 * t265 * t41794 / 3.0;
    (t42416, t42417, t42422, t42424, t42430, t42488)
}
