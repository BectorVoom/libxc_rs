//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 819/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk819<F: Float>(t1632: F, t2526: F, t551: F, t566: F, t2183: F, t2666: F, t2191: F, t2667: F, t2123: F, t538: F, t2625: F, t495: F) -> (F, F, F, F, F, F) {
    let t7309 = t1632 * t2526;
    let t7310 = t551 * t7309;
    let t7312 = F::new(0.69345773920434148506e0) * t566 * t7310;
    let t7313 = t2183 * t2666;
    let t7317 = F::new(0.23115257973478049502e0) * t2667 * t2191;
    let t7321 = t2123 * t538;
    let t7322 = t2625 * t495;
    (t7309, t7312, t7313, t7317, t7321, t7322)
}
