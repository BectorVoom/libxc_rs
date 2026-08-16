//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta382 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1723;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1724;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta382(t12166: f64, t378: f64, t342: f64, t11631: f64, t12050: f64, t12077: f64, t3154: f64, t12046: f64, t1647: f64, t3316: f64, t1071: f64, t4746: f64, t15669: f64, t379: f64, t994: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t16551, t16552, t16553, t16558, t16559, t16560, t16565, t16566, t16584, t16597) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1723(t12166, t378, t342, t11631, t12050, t12077, t3154, t12046, t1647, t3316, t1071, t4746);
        let (t16600, t16603) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1724(t15669, t378, t379, t994);
    (t16551, t16552, t16553, t16558, t16559, t16560, t16565, t16566, t16584, t16597, t16600, t16603)
}
