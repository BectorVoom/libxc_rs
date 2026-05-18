//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1269/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1269<F: Float>(t11104: F, t2156: F, t1147: F, t1306: F, t26780: F, t2993: F, t30767: F, t30769: F, t30772: F, t30775: F, t30778: F, t30780: F, t30784: F, t30786: F, t803: F, t9725: F) -> F {
    let t31014 = t11104 * t2156;
    let t31017 = -F::new(3.0) * t1147 * t1306 * t26780 - F::new(3.0) * t1306 * t2993 * t9725 - t1306 * t31014 * t803 - t30767 + t30769 - t30772 - t30775 + t30778 + t30780 - t30784 - t30786;
    t31017
}
