//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1074/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1074<F: Float>(t145607: F, t145611: F, t145615: F, t145619: F, t145621: F, t145626: F, t145628: F, t145632: F, t145636: F, t145640: F, t145644: F, t145648: F, t145652: F, t145656: F, t145661: F, t145663: F) -> F {
    let t145893 = t145607 / F::cast_from(18.0_f64) - t145611 / F::cast_from(6.0_f64) - t145615 / F::cast_from(8.0_f64) - F::cast_from(2.0_f64) * t145619 + t145621 / F::cast_from(27.0_f64) + t145626 / F::cast_from(18.0_f64) - t145628 / F::cast_from(27.0_f64) - t145632 / F::cast_from(6.0_f64) - F::cast_from(4.0_f64) * t145636 + F::cast_from(8.0_f64) * t145640 - F::cast_from(4.0_f64) * t145644 - F::cast_from(2.0_f64) * t145648 + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t145652 + t145656 / F::cast_from(9.0_f64) + t145661 / F::cast_from(3.0_f64) - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t145663;
    t145893
}
