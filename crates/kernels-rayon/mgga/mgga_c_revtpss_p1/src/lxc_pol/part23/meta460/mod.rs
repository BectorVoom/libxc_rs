//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta460 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1898;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta460(t16199: f64, t19661: f64, t1042: f64, t1469: f64, t4186: f64, t4806: f64, t16208: f64, t1065: f64, t6258: f64, t906: f64, t5825: f64, t606: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t19662, t19663, t19666, t19667, t19668, t19671, t19672, t19675, t19676, t19677, t19680) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1898(t16199, t19661, t1042, t1469, t4186, t4806, t16208, t1065, t6258, t906, t5825, t606);
    (t19662, t19663, t19666, t19667, t19668, t19671, t19672, t19675, t19676, t19677, t19680)
}
