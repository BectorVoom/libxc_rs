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
    let t145893 = t145607 / F::new(18.0) - t145611 / F::new(6.0) - t145615 / F::new(8.0) - F::new(2.0) * t145619 + t145621 / F::new(27.0) + t145626 / F::new(18.0) - t145628 / F::new(27.0) - t145632 / F::new(6.0) - F::new(4.0) * t145636 + F::new(8.0) * t145640 - F::new(4.0) * t145644 - F::new(2.0) * t145648 + F::new(2.0) / F::new(9.0) * t145652 + t145656 / F::new(9.0) + t145661 / F::new(3.0) - F::new(2.0) / F::new(9.0) * t145663;
    t145893
}
