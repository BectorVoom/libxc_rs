//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 699/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk699<F: Float>(t11034: F, t1866: F, t446: F, t1588: F, t7241: F, t942: F, t28: F, t89: F, t7773: F, t921: F, t1570: F, t1559: F) -> (F, F, F, F) {
    let t11035 = t1866 * t11034;
    let t11036 = t446 * t11035;
    let t11039 = t7241 * t942 * t1588;
    let t11041 = t89 * t28 * t11039;
    let t11043 = t89 * t7773 * t921;
    let t11045 = t942 * t1570;
    let t11046 = t11045 * t1559;
    (t11036, t11041, t11043, t11046)
}
