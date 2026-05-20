//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1911/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1911<F: Float>(t786: F, t97961: F, t1444: F, t5675: F, t25898: F, t98040: F, t1907: F, t3889: F, t25081: F, t7897: F, t1518: F, t2319: F) -> (F, F, F, F, F, F) {
    let t98308 = t786 * t97961;
    let t98362 = t5675 * t1444;
    let t98380 = t98040 * t25898;
    let t98436 = t1907 * t3889;
    let t98450 = t7897 * t25081;
    let t98484 = t2319 * t1518;
    (t98308, t98362, t98380, t98436, t98450, t98484)
}
