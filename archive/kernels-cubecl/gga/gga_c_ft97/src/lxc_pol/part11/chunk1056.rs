//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1056/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1056<F: Float>(t713: F, t9596: F, t2354: F, t446: F, t41490: F, t724: F, t2594: F, t41473: F, t2373: F, t2409: F, t9770: F, t505: F, t668: F, t9692: F) -> (F, F, F, F, F, F, F) {
    let t41930 = t9596 * t713;
    let t41932 = t446 * t2354 * t41930;
    let t41935 = t446 * t724 * t41490;
    let t41938 = t446 * t2594 * t41473;
    let t41940 = t2409 * t2373;
    let t41942 = t446 * t9770 * t41940;
    let t41945 = t9692 * t668 * t505;
    (t41930, t41932, t41935, t41938, t41940, t41942, t41945)
}
