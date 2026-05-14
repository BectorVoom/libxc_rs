//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 593/887 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk593<F: Float>(t100: F, t8326: F, t104: F, t7943: F, t89: F, t1786: F, t488: F, t7954: F, t82: F, t177: F, t2280: F, t1736: F, t70: F, t11: F, t2247: F, t41: F) -> (F, F, F, F, F, F, F) {
    let t8518 = t8326 * t100;
    let t8534 = 28.0 / 81.0 * t89 * t7943 * t104;
    let t8557 = t1786 * t488;
    let t8577 = t7954 * t82;
    let t8618 = 1.0 / t2280 / t177;
    let t8633 = t70 * t1736;
    let t8639 = t11 * t2247;
    let t8640 = t41 * t8639;
    (t8518, t8534, t8557, t8577, t8618, t8633, t8640)
}
