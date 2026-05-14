//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1200/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1200<F: Float>(t101718: F, t25883: F, t379: F, t93378: F, t93379: F, t1755: F, t1871: F, t22952: F, t5675: F, t965: F, t1900: F, t6: F, t8345: F, t91: F, t358: F, t100453: F, t100454: F) -> (F, F, F, F, F) {
    let t101719 = 2.0 / 27.0 * t101718;
    let t101724 = t93378 * t93379 * t25883 * t379;
    let t101729 = t22952 * t1871 * t5675 * t965 * t1755;
    let t101733 = t91 * t8345 * t6 * t1900;
    let t101734 = t965 * t358;
    let t101737 = t101733 * t100453 * t101734 * t100454;
    (t101719, t101724, t101729, t101734, t101737)
}
