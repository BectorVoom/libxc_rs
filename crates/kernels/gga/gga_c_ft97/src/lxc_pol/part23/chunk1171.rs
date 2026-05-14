//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1171/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1171<F: Float>(t113346: F, t25162: F, t28752: F, t92: F, t99475: F, t28748: F, t1882: F, t28813: F, t43912: F, t6318: F, t1486: F, t2399: F, t7071: F, t6308: F, t7063: F, t1636: F, t7087: F, t89: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t113347 = t113346 / 6.0;
    let t113348 = t25162 * t28752;
    let t113349 = t113348 / 9.0;
    let t113350 = t99475 * t92;
    let t113356 = t25162 * t28748;
    let t113357 = t113356 / 9.0;
    let t113372 = t1882 * t28813;
    let t113373 = 4.0 / 9.0 * t113372;
    let t113374 = t43912 * t6318;
    let t113379 = t1486 * t2399 * t7071;
    let t113386 = t6308 * t2399 * t7063;
    let t113420 = t89 * t1636 * t7087;
    (t113347, t113348, t113349, t113350, t113356, t113357, t113372, t113373, t113374, t113379, t113386, t113420)
}
