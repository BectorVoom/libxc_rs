//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 590/869 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk590<F: Float>(t2252: F, t342: F, t511: F, t1526: F, t1944: F, t7705: F, t1948: F, t630: F, t142: F, t1557: F, t1559: F, t1570: F, t1580: F, t1943: F, t2075: F, t72: F) -> (F, F, F, F, F, F, F) {
    let t8759 = t342 * t2252 * t511 / 18.0;
    let t8761 = t1526 * t7705 * t1944;
    let t8764 = t342 * t630 * t1948;
    let t8766 = t142 * t1557;
    let t8767 = t8766 * t1559;
    let t8774 = t142 * t1570;
    let t8775 = t8774 * t1559;
    let t8779 = t1943 * t1580;
    let t8783 = t72 * t2075;
    (t8759, t8761, t8764, t8767, t8775, t8779, t8783)
}
