//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 922/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk922<F: Float>(t145585: F, t27: F, t370: F, t89: F, t34507: F, t375: F, t144813: F, t38262: F, t446: F, t144801: F, t7824: F, t32063: F, t34385: F, t7238: F, t137163: F, t137172: F, t137180: F, t145038: F, t145042: F, t145045: F, t145048: F, t145051: F, t145055: F, t145058: F, t145061: F) -> (F, F, F, F, F, F) {
    let t145588 = t89 * t27 * t370 * t145585;
    let t145590 = t89 * t375 * t34507;
    let t145595 = t446 * t38262 * t144813;
    let t145598 = t446 * t7824 * t144801;
    let t145601 = t7238 * t32063 * t34385;
    let t145603 = t145038 + t145042 / 4.0 - t145045 - 3.0 * t145048 - 2.0 / 3.0 * t145051 + 2.0 / 9.0 * t137163 + 4.0 / 3.0 * t145055 - 4.0 / 9.0 * t145058 + t145061 / 6.0 - t145588 + t145590 / 3.0 + 4.0 / 3.0 * t137172 - 8.0 / 3.0 * t137180 + 2.0 * t145595 - 4.0 / 3.0 * t145598 + 4.0 / 3.0 * t145601;
    (t145588, t145590, t145595, t145598, t145601, t145603)
}
