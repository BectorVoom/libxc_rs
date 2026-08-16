//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 571/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk571<F: Float>(t135: F, t2006: F, t2011: F, t2021: F, t2082: F, t3325: F, t3358: F, t3437: F, t4616: F, t4620: F, t4626: F, t4631: F, t4652: F, t4656: F, t4661: F, t4665: F, t628: F, t636: F) -> F {
    let t4668 = t2006 + F::cast_from(7.0_f64) / F::cast_from(72.0_f64) * t3325 + t2011 * t4616 / F::cast_from(16.0_f64) - t628 * t4620 / F::cast_from(48.0_f64) + F::cast_from(0.54332259311179736592e-2_f64) * t2021 * t4626 + F::cast_from(0.2535505434521721041e-1_f64) * t3358 + F::cast_from(0.21732903724471894636e-1_f64) * t636 * t4631 - F::cast_from(0.27166129655589868296e-2_f64) * t636 * t4652 - F::cast_from(0.27166129655589868296e-2_f64) * t636 * t4656 + t2082 + F::cast_from(0.10142021738086884164e0_f64) * t3437 + F::cast_from(0.5433225931117973659e-1_f64) * t135 * t4661 - F::cast_from(0.10866451862235947318e-1_f64) * t135 * t4665;
    t4668
}
