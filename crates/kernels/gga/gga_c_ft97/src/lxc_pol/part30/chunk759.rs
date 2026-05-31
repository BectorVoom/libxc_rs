//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 759/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk759<F: Float>(t681: F, t7490: F, t1403: F, t7441: F, t771: F, t193: F, t33452: F, t675: F, t263: F, t7486: F, t7442: F, t5999: F, t7437: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t33571 = t681 * t7490;
    let t33573 = t1403 * t33571 / F::cast_from(9.0_f64);
    let t33574 = t7441 * t771;
    let t33575 = t193 * t33574;
    let t33582 = t675 * t33452;
    let t33583 = t33582 * t263;
    let t33584 = t193 * t33583;
    let t33587 = t681 * t7486;
    let t33589 = t1403 * t33587 / F::cast_from(18.0_f64);
    let t33590 = t681 * t7442;
    let t33592 = t1403 * t33590 / F::cast_from(9.0_f64);
    let t33594 = t7437 * t5999 / F::cast_from(18.0_f64);
    (t33571, t33573, t33574, t33575, t33582, t33583, t33584, t33587, t33589, t33590, t33592, t33594)
}
