//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 786/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk786<F: Float>(t34042: F, t34045: F, t34049: F, t35861: F, t35866: F, t35975: F, t35979: F, t35983: F, t35987: F, t35991: F, t35995: F, t35999: F, t36033: F) -> (F,) {
    let t36041 = t34042 + t35861 / 6.0 + t35866 - t35975 / 2.0 - t34045 - 2.0 / 3.0 * t35979 - 6.0 * t35983 + 4.0 * t35987 + t34049 + t35991 / 3.0 + 2.0 * t35995 - t35999;
    let t36042 = t36033 + t36041;
    (t36042,)
}
