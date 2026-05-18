//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 784/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk784<F: Float>(t21428: F, t21462: F, t661: F, t1168: F, t4934: F, t2574: F, t762: F, t1131: F, t5053: F, t265: F, t3977: F, t5073: F, t729: F) -> (F, F, F, F, F, F, F) {
    let t21463 = t21428 + t21462;
    let t21464 = t661 * t21463;
    let t21472 = t4934 * t1168;
    let t21474 = t2574 * t762 * t21472;
    let t21477 = t1131 * t5053;
    let t21479 = t2574 * t265 * t21477;
    let t21483 = t729 * t3977 * t5073;
    (t21463, t21464, t21472, t21474, t21477, t21479, t21483)
}
