//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 643/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk643<F: Float>(t3746: F, t6161: F, t2606: F, t3837: F, t13885: F, t24668: F, t3842: F, t14127: F, t11593: F, t1901: F, t24742: F, t24757: F, t28326: F, t28330: F, t28334: F, t28338: F, t28341: F, t28346: F, t28350: F, t28353: F, t28357: F, t446: F) -> (F, F, F, F) {
    let t28360 = t6161 * t3746;
    let t28361 = t2606 * t28360;
    let t28364 = t6161 * t3837;
    let t28365 = t13885 * t28364;
    let t28368 = t24668 * t3842;
    let t28369 = t14127 * t28368;
    let t28372 = F::new(2.0) / F::new(3.0) * t446 * t28326 - t446 * t28330 / F::new(9.0) + t446 * t28334 / F::new(3.0) - t24742 / F::new(27.0) - F::new(2.0) / F::new(9.0) * t28338 - t24757 - F::new(2.0) / F::new(9.0) * t1901 * t28341 + F::new(2.0) / F::new(27.0) * t1901 * t28346 - t1901 * t28350 / F::new(9.0) - t28353 / F::new(27.0) + t1901 * t28357 / F::new(9.0) - F::new(2.0) / F::new(9.0) * t11593 * t28361 - F::new(2.0) / F::new(3.0) * t1901 * t28365 - F::new(2.0) / F::new(3.0) * t1901 * t28369;
    (t28360, t28364, t28368, t28372)
}
