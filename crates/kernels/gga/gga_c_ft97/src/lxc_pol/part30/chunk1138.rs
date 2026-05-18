//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1138/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1138<F: Float>(t10409: F, t152888: F, t446: F, t1486: F, t35849: F, t681: F, t152776: F, t33829: F, t7512: F, t7638: F, t152780: F, t7641: F) -> (F, F, F, F) {
    let t153453 = t446 * t10409 * t152888;
    let t153456 = t1486 * t681 * t35849;
    let t153460 = t7638 * t7512 * t33829 * t152776;
    let t153464 = t7638 * t7512 * t7641 * t152780;
    (t153453, t153456, t153460, t153464)
}
