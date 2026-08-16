//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta381 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1722;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta381(t1071: f64, t3316: f64, t342: f64, t1647: f64, t3298: f64, t4980: f64, t989: f64, t4995: f64, t1086: f64, t1678: f64, t994: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t16505, t16506, t16509, t16520, t16523, t16543, t16544) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1722(t1071, t3316, t342, t1647, t3298, t4980, t989, t4995, t1086, t1678, t994);
    (t16505, t16506, t16509, t16520, t16523, t16543, t16544)
}
