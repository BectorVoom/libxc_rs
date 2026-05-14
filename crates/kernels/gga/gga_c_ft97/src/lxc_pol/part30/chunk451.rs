//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 451/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk451<F: Float>(t1491: F, t461: F, t1476: F, t231: F, t2: F, t342: F, t343: F, t7570: F, t4: F, t26: F) -> (F, F, F, F) {
    let t7571 = t461 * t1491;
    let t7574 = t231 * t1476;
    let t7579 = (-t7570 * t7571 / 6.0 - t342 * t343 * t7574 / 4.0) * t2;
    let t7580 = t7579 * t4;
    let t7581 = t7580 * t26;
    (t7571, t7574, t7580, t7581)
}
