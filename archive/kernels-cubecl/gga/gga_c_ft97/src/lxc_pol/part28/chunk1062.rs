//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1062/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1062<F: Float>(t137231: F, t1564: F, t446: F, t920: F, t32325: F, t942: F, t1317: F, t1800: F, t28: F, t34401: F, t376: F, t145607: F, t145611: F, t145615: F, t145619: F, t145621: F, t145626: F, t145628: F, t145632: F, t145636: F, t145640: F, t145644: F, t145648: F, t145652: F) -> (F, F, F, F, F) {
    let t145656 = t446 * t1564 * t137231 * t920;
    let t145658 = t32325 * t942;
    let t145661 = t1317 * t28 * t1800 * t145658;
    let t145663 = t1317 * t376 * t34401;
    let t145665 = t145607 / F::cast_from(6.0_f64) - t145611 / F::cast_from(2.0_f64) - F::cast_from(3.0_f64) / F::cast_from(8.0_f64) * t145615 - F::cast_from(6.0_f64) * t145619 + t145621 / F::cast_from(9.0_f64) + t145626 / F::cast_from(6.0_f64) - t145628 / F::cast_from(9.0_f64) - t145632 / F::cast_from(2.0_f64) - F::cast_from(12.0_f64) * t145636 + F::cast_from(24.0_f64) * t145640 - F::cast_from(12.0_f64) * t145644 - F::cast_from(6.0_f64) * t145648 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t145652 + t145656 / F::cast_from(3.0_f64) + t145661 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t145663;
    (t145656, t145658, t145661, t145663, t145665)
}
