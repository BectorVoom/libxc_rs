//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1345/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1345<F: Float>(t31551: F, t824: F, t1486: F, t193: F, t2781: F, t113252: F, t113253: F, t113269: F, t113270: F, t113273: F, t113296: F, t126832: F, t126835: F, t126839: F, t126844: F, t10248: F, t126389: F, t446: F) -> (F, F, F, F) {
    let t126846 = t31551 * t824;
    let t126849 = t1486 * t193 * t2781 * t126846;
    let t126851 = -t113252 + 8.0 / 27.0 * t113253 + t113269 + 4.0 / 9.0 * t113270 + t126832 + 10.0 / 27.0 * t126835 - 8.0 / 9.0 * t126839 + t126844 / 4.0 + t126849 - 4.0 / 9.0 * t113273 - t113296;
    let t126854 = t446 * t10248 * t126389;
    (t126846, t126849, t126851, t126854)
}
