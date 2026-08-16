//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 807/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk807<F: Float>(t113: F, t21801: F, t1273: F, t5473: F, t4381: F, t1274: F, t4635: F, t5479: F, t992: F, t5474: F, t10304: F, t21130: F) -> (F, F, F, F, F, F, F) {
    let t21802 = t21801 * t113;
    let t21805 = t5473 * t1273;
    let t21806 = t21805 * t4381;
    let t21812 = t1274 * t4635;
    let t21815 = t5479 * t992;
    let t21818 = t5474 * t992;
    let t21821 = t10304 * t21130;
    (t21802, t21805, t21806, t21812, t21815, t21818, t21821)
}
