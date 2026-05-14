//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 979/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk979<F: Float>(t762: F, t9952: F, t1095: F, t806: F, t9608: F, t1611: F, t1609: F, t9523: F, t2378: F, t3750: F, t224: F, t6: F, t9682: F, t51: F, t6041: F, t3771: F) -> (F, F, F, F, F, F, F, F, F) {
    let t52054 = t9952 * t762;
    let t52263 = t806 * t1095;
    let t52267 = t9608 * t1095;
    let t52324 = t1611 * t806;
    let t52358 = t1609 * t9523;
    let t52369 = t2378 * t3750;
    let t52385 = t9523 * t1095;
    let t52588 = t224 * t9682 * t6;
    let t52593 = t6041 * t51;
    let t52594 = t3771 * t52593;
    (t52054, t52263, t52267, t52324, t52358, t52369, t52385, t52588, t52594)
}
