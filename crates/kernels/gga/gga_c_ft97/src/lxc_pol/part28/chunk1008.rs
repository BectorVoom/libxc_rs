//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1008/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1008<F: Float>(t1369: F, t34919: F, t376: F, t148249: F, t2112: F, t28: F, t148412: F, t446: F, t9073: F, t148417: F, t39693: F, t32063: F, t32888: F, t34809: F, t34918: F, t558: F) -> (F, F, F, F, F, F) {
    let t148660 = t1369 * t376 * t34919;
    let t148667 = t1369 * t28 * t2112 * t148249;
    let t148670 = t446 * t9073 * t148412;
    let t148673 = t446 * t39693 * t148417;
    let t148676 = t32888 * t32063 * t34809;
    let t148678 = t34918 * t558;
    (t148660, t148667, t148670, t148673, t148676, t148678)
}
