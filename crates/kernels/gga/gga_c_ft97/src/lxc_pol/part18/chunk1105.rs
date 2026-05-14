//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1105/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1105<F: Float>(t5675: F, t8216: F, t1570: F, t5617: F, t1882: F, t22983: F, t1307: F, t7763: F, t22995: F, t1317: F, t1637: F, t5684: F, t1557: F, t22862: F, t358: F, t22988: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t93379 = t8216 * t5675;
    let t93409 = t5617 * t1570;
    let t93414 = t1882 * t22983;
    let t93415 = t93414 / 9.0;
    let t93416 = t1307 * t7763;
    let t93421 = t1882 * t22995;
    let t93422 = 2.0 / 9.0 * t93421;
    let t93424 = t1317 * t1637 * t5684;
    let t93434 = t5617 * t1557;
    let t93440 = t22862 * t358;
    let t93449 = t1882 * t22988;
    (t93379, t93409, t93414, t93415, t93416, t93421, t93422, t93424, t93434, t93440, t93449)
}
