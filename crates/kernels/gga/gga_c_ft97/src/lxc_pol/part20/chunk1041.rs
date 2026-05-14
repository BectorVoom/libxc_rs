//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1041/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1041<F: Float>(t1486: F, t25022: F, t681: F, t25018: F, t6308: F, t1882: F, t24997: F, t2399: F, t6339: F, t89: F, t24965: F, t25198: F, t1483: F, t3281: F, t25268: F, t25263: F) -> (F, F, F, F, F, F, F, F, F) {
    let t99584 = t1486 * t681 * t25022;
    let t99599 = t6308 * t681 * t25018;
    let t99601 = t1882 * t24997;
    let t99607 = t89 * t2399 * t6339;
    let t99610 = t89 * t681 * t24965;
    let t99628 = t1882 * t25198;
    let t99635 = 28.0 / 81.0 * t3281 * t1483;
    let t99644 = t1882 * t25268;
    let t99646 = t1882 * t25263;
    (t99584, t99599, t99601, t99607, t99610, t99628, t99635, t99644, t99646)
}
