//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1206/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1206<F: Float>(t2801: F, t2843: F, t7124: F, t1466: F, t2399: F, t7023: F, t6967: F, t98388: F, t25462: F, t28993: F, t1253: F, t14889: F, t193: F, t24964: F, t25435: F, t25438: F, t25480: F, t28966: F, t28968: F, t29047: F, t29416: F, t317: F, t4129: F, t44351: F, t6210: F, t6222: F, t6263: F, t6963: F, t7028: F, t880: F) -> (F, F) {
    let t112627 = t2843 * t7124 * t2801;
    let t112630 = t1466 * t2399 * t7023;
    let t112641 = t98388 * t6967 / 27.0;
    let t112643 = t25462 * t28993 / 27.0;
    let t112644 = t25480 * t7028 / 6.0 + t29416 * t6263 / 3.0 - 2.0 / 3.0 * t6210 * t28968 - 2.0 / 3.0 * t1466 * t193 * t24964 * t28966 - 2.0 / 3.0 * t1466 * t193 * t6222 * t880 * t4129 - t1466 * t193 * t6222 * t317 * t14889 / 3.0 + 4.0 * t112627 + 2.0 / 27.0 * t112630 - 24.0 * t44351 * t29047 + t6963 * t25435 / 3.0 + t1466 * t193 * t25438 * t1253 / 6.0 + t112641 + t112643;
    (t112627, t112644)
}
