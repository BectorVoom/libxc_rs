//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 877/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk877<F: Float>(t193: F, t36012: F, t1253: F, t7612: F, t34031: F, t34036: F, t35822: F, t35826: F, t35831: F, t35836: F, t35840: F, t35844: F, t35848: F, t35851: F, t35856: F) -> (F, F, F, F) {
    let t36013 = t193 * t36012;
    let t36016 = t7612 * t1253;
    let t36017 = t193 * t36016;
    let t36033 = F::new(3.0) / F::new(2.0) * t35822 + t34031 + F::new(2.0) / F::new(3.0) * t35826 + F::new(4.0) * t35831 - F::new(2.0) * t35836 - t35840 / F::new(2.0) - t34036 - t35844 / F::new(3.0) - F::new(3.0) * t35848 + F::new(2.0) * t35851 + t35856 / F::new(4.0);
    (t36013, t36016, t36017, t36033)
}
