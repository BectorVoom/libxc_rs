//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta599 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2485;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2486;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta599(t3022: f64, t6219: f64, t6223: f64, t2986: f64, t6205: f64, t974: f64, t981: f64, t4708: f64, t4724: f64, t3336: f64, t6396: f64, t6184: f64, t964: f64, t19021: f64, t973: f64, t11461: f64, t11554: f64, t15343: f64, t1634: f64, t19029: f64, t19031: f64, t19058: f64, t19060: f64, t19062: f64, t2982: f64, t4685: f64, t6190: f64, t6206: f64, t6209: f64, t965: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t19143, t19145, t19146, t19147, t19149, t19150, t19152, t19153, t19156) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2485(t3022, t6219, t6223, t2986, t6205, t974, t981, t4708, t4724, t3336, t6396, t6184, t964);
        let (t19167, t19172) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2486(t19021, t973, t11461, t11554, t15343, t1634, t19029, t19031, t19058, t19060, t19062, t19156, t2982, t4685, t4708, t6190, t6206, t6209, t965, t974);
    (t19143, t19145, t19146, t19147, t19149, t19150, t19152, t19153, t19156, t19167, t19172)
}
