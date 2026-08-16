//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 634/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk634<F: Float>(t1456: F, t2574: F, t3837: F, t3746: F, t724: F, t1091: F, t6194: F, t242: F, t28100: F, t1901: F, t24605: F, t24611: F, t28195: F, t28198: F, t28201: F, t28205: F, t28209: F, t28212: F, t28214: F, t3281: F, t446: F) -> F {
    let t28218 = t2574 * t1456 * t3837;
    let t28222 = t724 * t1456 * t3746;
    let t28226 = t724 * t6194 * t1091;
    let t28230 = t242 * t28100;
    let t28233 = -t446 * t28195 / F::cast_from(3.0_f64) - t446 * t28198 / F::cast_from(3.0_f64) - t446 * t28201 / F::cast_from(3.0_f64) - t1901 * t28205 / F::cast_from(9.0_f64) - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t1901 * t28209 + t28212 / F::cast_from(9.0_f64) + t28214 / F::cast_from(9.0_f64) - t24605 / F::cast_from(9.0_f64) + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t28218 + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t3281 * t28222 - t446 * t28226 / F::cast_from(9.0_f64) - t24611 / F::cast_from(9.0_f64) - t446 * t28230 / F::cast_from(3.0_f64);
    t28233
}
