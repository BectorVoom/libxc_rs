//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 712/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk712<F: Float>(t28848: F, t296: F, t4246: F, t6365: F, t840: F, t28850: F, t1501: F, t4129: F, t871: F, t25246: F, t25248: F, t25252: F, t25284: F, t29340: F, t29342: F, t29346: F, t29350: F, t29354: F, t29356: F, t446: F) -> (F, F) {
    let t29359 = t296 * t28848;
    let t29363 = t840 * t4246 * t6365;
    let t29366 = t296 * t28850;
    let t29369 = t1501 * t4129;
    let t29371 = t840 * t871 * t29369;
    let t29374 = -F::new(2.0) / F::new(9.0) * t25246 - t25248 / F::new(9.0) + t25252 + t29340 / F::new(9.0) - t446 * t29342 / F::new(3.0) + t446 * t29346 / F::new(3.0) + F::new(2.0) / F::new(3.0) * t446 * t29350 - t25284 / F::new(27.0) + t29354 / F::new(27.0) + F::new(2.0) / F::new(3.0) * t446 * t29356 + F::new(2.0) / F::new(3.0) * t446 * t29359 + t446 * t29363 / F::new(3.0) + F::new(2.0) / F::new(3.0) * t446 * t29366 + t446 * t29371 / F::new(3.0);
    (t29369, t29374)
}
