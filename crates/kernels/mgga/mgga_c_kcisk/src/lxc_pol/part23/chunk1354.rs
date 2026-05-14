//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1354/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1354<F: Float>(t1339: F, t19998: F, t32045: F, t18967: F, t110290: F, t110351: F, t110524: F, t113663: F, t113729: F, t113735: F, t113740: F, t113747: F, t113749: F, t113761: F, t113765: F, t20229: F, t32008: F, t32087: F, t33360: F, t33428: F, t9426: F, t9446: F, t9796: F) -> (F, F, F) {
    let t113769 = t1339 * t32045 * t19998;
    let t113772 = t1339 * t32045 * t18967;
    let t113774 = -0.13888888888888888889e-1 * t32087 * t113735 * t20229 - 0.26805555555555555556e-2 * t32008 * t113740 - 0.18518518518518518519e-1 * t110524 * t33360 + t113747 - 0.33163888888888888888e-2 * t113749 + 0.40208333333333333335e-2 * t9426 * t113663 + 0.13402777777777777778e-2 * t110290 + 0.20833333333333333334e-1 * t110351 * t9796 - 0.18518518518518518519e-1 * t110524 * t33428 + 0.35740740740740740742e-2 * t32008 * t113729 + 0.1621345679012345679e-1 * t113761 - 0.20833333333333333334e-1 * t9446 * t113765 - 0.33163888888888888888e-2 * t113769 - 0.16581944444444444444e-2 * t113772;
    (t113769, t113772, t113774)
}
