//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 417/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk417<F: Float>(t2268: F, t787: F, t269: F, t348: F, t2059: F, t737: F, t2078: F, t257: F, t751: F, t105: F, t107: F, t1308: F, t1312: F, t1319: F, t2141: F, t260: F, t438: F, t446: F, t447: F, t780: F) -> (F, F) {
    let t2269 = t2268 * t787;
    let t2281 = t348 * t269;
    let t2287 = t737 * t2059;
    let t2291 = t257 * t2078;
    let t2295 = t751 * t751;
    let t2299 = -0.11281315546296296296e-3 * t105 * t1308 * t269 + 0.1e-22 * t446 * t1312 * t269 - 0.67687893277777777778e-3 * t105 * t438 * t780 + 0.50765919958333333334e-3 * t1319 * t2281 + 0.50765919958333333334e-3 * t446 * t447 * t780 + 0.10153183991666666667e-2 * t105 * t107 * t2287 - 0.50765919958333333334e-3 * t105 * t107 * t2291 - 4.0 * t2295 - 4.0 * t260 * t2141;
    (t2269, t2299)
}
