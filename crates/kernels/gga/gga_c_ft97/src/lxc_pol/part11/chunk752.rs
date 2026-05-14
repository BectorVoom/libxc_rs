//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 752/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk752<F: Float>(t10864: F, t668: F, t10915: F, t294: F, t2917: F, t2781: F, t1595: F, t7857: F, t11120: F, t1299: F, t388: F, t51: F, t5566: F, t1608: F, t35: F, t428: F) -> (F, F, F, F, F, F, F, F, F) {
    let t18862 = t10864 * t668;
    let t18961 = t10915 * t294;
    let t18968 = t2917 * t294;
    let t19714 = t2781 * t668;
    let t22547 = t7857 * t1595;
    let t22548 = t22547 * t11120;
    let t22590 = t388 * t1299;
    let t22602 = t5566 * t51;
    let t22603 = t1608 * t22602;
    let t22604 = t35 * t428;
    (t18862, t18961, t18968, t19714, t22547, t22548, t22590, t22603, t22604)
}
