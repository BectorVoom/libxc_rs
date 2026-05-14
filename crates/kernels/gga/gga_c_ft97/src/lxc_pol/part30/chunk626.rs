//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 626/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk626<F: Float>(t24974: F, t24987: F, t28722: F, t28727: F, t28732: F, t28739: F, t28744: F, t28749: F, t28753: F, t28758: F, t28762: F, t28765: F, t24995: F, t25010: F, t28770: F, t28774: F, t28779: F, t28783: F, t28784: F, t28790: F, t28794: F, t28798: F, t28802: F, t28805: F) -> (F, F) {
    let t28897 = -t28722 - t24974 / 12.0 - t28727 / 12.0 - t28732 / 12.0 - 2.0 / 3.0 * t24987 - 3.0 / 8.0 * t28739 - t28744 / 2.0 + t28749 / 6.0 + t28753 / 6.0 - t28758 / 3.0 - t28762 / 3.0 - t28765 / 3.0;
    let t28911 = -t28770 / 3.0 + t28774 / 9.0 - t28779 / 2.0 - 3.0 * t28783 - t28784 / 18.0 + t24995 / 3.0 - t25010 / 3.0 - 2.0 / 3.0 * t28790 - 6.0 * t28794 + t28798 / 3.0 - t28802 / 2.0 - 2.0 / 3.0 * t28805;
    (t28897, t28911)
}
