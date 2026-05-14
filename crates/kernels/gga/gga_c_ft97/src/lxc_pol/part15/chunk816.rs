//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 816/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk816<F: Float>(t1611: F, t236: F, t3771: F, t806: F, t5045: F, t626: F, t701: F, t5037: F, t5041: F, t13647: F, t3799: F, t228: F, t231: F, t4995: F, t625: F, t4947: F, t9608: F) -> (F, F, F, F, F, F, F) {
    let t65743 = t3771 * t236 * t1611 * t806;
    let t65850 = t701 * t626 * t5045;
    let t65853 = t701 * t626 * t5037;
    let t65860 = t701 * t626 * t5041;
    let t65862 = t3799 * t13647;
    let t65952 = t228 * t4995 * t625 * t231;
    let t66092 = t3771 * t4947 * t9608;
    (t65743, t65850, t65853, t65860, t65862, t65952, t66092)
}
