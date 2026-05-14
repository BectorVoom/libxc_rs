//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1069/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1069<F: Float>(t676: F, t754: F, t2567: F, t3951: F, t10002: F, t737: F, t2492: F, t773: F, t9895: F, t762: F, t9952: F, t1611: F, t806: F, t1609: F, t9523: F, t224: F, t6: F, t9682: F) -> (F, F, F, F, F, F, F, F, F) {
    let t51853 = t676 * t754;
    let t51892 = t3951 * t2567;
    let t51901 = t737 * t10002;
    let t52002 = t2492 * t3951;
    let t52018 = t9895 * t773;
    let t52054 = t9952 * t762;
    let t52324 = t1611 * t806;
    let t52358 = t1609 * t9523;
    let t52588 = t224 * t9682 * t6;
    (t51853, t51892, t51901, t52002, t52018, t52054, t52324, t52358, t52588)
}
