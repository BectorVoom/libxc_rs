//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 421/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk421<F: Float>(t6940: F, t762: F, t242: F, t1901: F, t193: F, t446: F, t6073: F, t6099: F, t6160: F, t6849: F, t6854: F, t6858: F, t6863: F, t6867: F, t6871: F, t6875: F, t6909: F, t6914: F, t6918: F, t6923: F, t6927: F, t6932: F, t89: F) -> (F, F) {
    let t6941 = t762 * t6940;
    let t6942 = t242 * t6941;
    let t6945 = t6073 + t1901 * t6849 / F::new(9.0) + F::new(2.0) / F::new(3.0) * t446 * t6854 - t446 * t6858 / F::new(3.0) + t446 * t6863 / F::new(3.0) - t446 * t6867 / F::new(3.0) - t6099 - t446 * t6871 / F::new(9.0) - t446 * t6875 / F::new(3.0) + t89 * t193 * t6909 / F::new(3.0) - t446 * t6914 / F::new(3.0) + t6160 + t1901 * t6918 / F::new(9.0) + t446 * t6923 / F::new(3.0) - t446 * t6927 / F::new(3.0) + F::new(2.0) / F::new(3.0) * t446 * t6932 - t446 * t6942 / F::new(3.0);
    (t6942, t6945)
}
