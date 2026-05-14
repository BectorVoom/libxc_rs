//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 558/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk558<F: Float>(t135: F, t2006: F, t2011: F, t2021: F, t2082: F, t3325: F, t3358: F, t3437: F, t4616: F, t4620: F, t4626: F, t4631: F, t4652: F, t4656: F, t4661: F, t4665: F, t628: F, t636: F) -> (F,) {
    let t4668 = t2006 + 7.0 / 72.0 * t3325 + t2011 * t4616 / 16.0 - t628 * t4620 / 48.0 + 0.54332259311179736592e-2 * t2021 * t4626 + 0.2535505434521721041e-1 * t3358 + 0.21732903724471894636e-1 * t636 * t4631 - 0.27166129655589868296e-2 * t636 * t4652 - 0.27166129655589868296e-2 * t636 * t4656 + t2082 + 0.10142021738086884164e0 * t3437 + 0.5433225931117973659e-1 * t135 * t4661 - 0.10866451862235947318e-1 * t135 * t4665;
    (t4668,)
}
