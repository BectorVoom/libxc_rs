//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 574/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk574<F: Float>(t2354: F, t27483: F, t446: F, t1411: F, t3758: F, t1109: F, t709: F, t444: F, t6032: F, t3789: F, t2446: F, t3886: F, sigma2: F) -> (F, F, F, F, F, F, F) {
    let t27484 = t2354 * t27483;
    let t27485 = t446 * t27484;
    let t27487 = t3758 * t1411;
    let t27494 = sigma2 * t1109;
    let t27495 = t27494 * t709;
    let t27499 = t6032 * t444;
    let t27500 = t3789 * t27499;
    let t27501 = t2446 * t3886;
    (t27485, t27487, t27494, t27495, t27499, t27500, t27501)
}
