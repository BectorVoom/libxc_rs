//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta922 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3144;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta922(t1276: f64, t5245: f64, t460: f64, t488: f64, t13181: f64, t1828: f64, t12627: f64, t12626: f64, t1769: f64, t487: f64, t1770: f64, t3727: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t56310, t56314, t56315, t56327, t56331, t56332, t56384) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3144(t1276, t5245, t460, t488, t13181, t1828, t12627, t12626, t1769, t487, t1770, t3727);
    (t56310, t56314, t56315, t56327, t56331, t56332, t56384)
}
