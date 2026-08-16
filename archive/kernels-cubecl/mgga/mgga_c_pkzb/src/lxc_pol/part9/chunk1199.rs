//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1199/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1199<F: Float>(t20757: F, t20819: F, t713: F, t722: F, t730: F, t5839: F, t7226: F, t1980: F, t2848: F, t5498: F, t1107: F, t5838: F) -> (F, F, F, F, F) {
    let t20820 = t20757 + t20819;
    let t20824 = F::cast_from(0.5848223622634646207e0_f64) * t730 * t713 * t20820 * t722;
    let t20827 = F::cast_from(0.6233709278045326953e3_f64) * t730 * t7226 * t5839;
    let t20831 = F::cast_from(0.31168546390226634765e3_f64) * t730 * t5498 * t2848 * t1980;
    let t20834 = t5838 * t1107;
    (t20820, t20824, t20827, t20831, t20834)
}
