//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta77 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk495;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk496;
use chunk2::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk497;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta77<F: Float>(t45: F, t57: F, t1469: F, t190: F, t706: F, t78: F, t81: F, t150: F, t162: F, t187: F, t766: F, t770: F, zeta_threshold: F, t124: F, t800: F, t225: F, t679: F, t704: F, t751: F, t759: F, t764: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t1522, t1524, t1531, t1532, t1533, t1534, t1536, t1544) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk495::<F>(t45, t57, t1469, t190, t706, t78, t81, t150, t162, t187, t766, t770, zeta_threshold);
        let (t1548, t1549) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk496::<F>(t124, t1544, t800);
        let t1553 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk497::<F>(t1524, t1533, t1536, t225, t679, t704, t751, t759, t764);
    (t1522, t1524, t1531, t1532, t1533, t1534, t1536, t1544, t1548, t1549, t1553)
}
