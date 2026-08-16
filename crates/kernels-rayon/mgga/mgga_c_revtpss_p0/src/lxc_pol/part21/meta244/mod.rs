//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta244 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1423;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1424;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta244(t1317: f64, t3855: f64, t4029: f64, t1333: f64, t3863: f64, t27: f64, t583: f64, t521: f64, t19: f64, t596: f64, t182: f64, t2490: f64, t2495: f64, t9368: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t9404, t9405, t9407, t9408, t9409, t9410, t9411, t9412, t9413, t9415, t9417) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1423(t1317, t3855, t4029, t1333, t3863, t27, t583, t521, t19, t596, t182, t2490);
        let t9419 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1424(t2495, t9368, t9417);
    (t9404, t9405, t9407, t9408, t9409, t9410, t9411, t9412, t9413, t9415, t9417, t9419)
}
