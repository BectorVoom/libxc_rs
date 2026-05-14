//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1058/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1058<F: Float>(t1218: F, t21930: F, t22469: F, t312: F, t317: F, t5304: F, t5305: F, t5422: F, t788: F, t90775: F, t90785: F, t90803: F, t90873: F, t90936: F, t91005: F, t91082: F, t91125: F, t91158: F, t91171: F, t91186: F, t91195: F) -> (F,) {
    let t91216 = -t788 * (t91158 + t91171 + t91186 + t91195) * t317 + 12.0 * t91125 + 48.0 * t90936 - 72.0 * t90873 - 12.0 * t90775 + 16.0 * t90785 - 3.0 * t21930 * t5304 * t317 + 2.0 * t91082 * t312 + 48.0 * t91005 - 48.0 * t90803 - 6.0 * t5305 * t5422 - 4.0 * t1218 * t22469;
    (t91216,)
}
