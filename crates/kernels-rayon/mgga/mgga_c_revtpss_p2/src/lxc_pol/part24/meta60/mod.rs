//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta60 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk386;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk387;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk388;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta60(t45: f64, t57: f64, t1518: f64, t508: f64, t1469: f64, t190: f64, t706: f64, t78: f64, t81: f64, zeta_threshold: f64, t150: f64, t162: f64, t187: f64, t766: f64, t770: f64, t124: f64, t800: f64, t225: f64, t679: f64, t704: f64, t751: f64, t759: f64, t764: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t1519, t1522, t1524, t1531) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk386(t45, t57, t1518, t508, t1469, t190, t706, t78, t81, zeta_threshold);
        let (t1532, t1533, t1534, t1536, t1544) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk387(t45, t57, t150, t1531, t190, t162, t187, t1469, t766, t770, zeta_threshold);
        let (t1549, t1553) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk388(t124, t1544, t800, t1524, t1533, t1536, t225, t679, t704, t751, t759, t764);
    (t1519, t1522, t1524, t1531, t1532, t1533, t1534, t1536, t1544, t1549, t1553)
}
