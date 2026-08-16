//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta839 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3146;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3147;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3148;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta839(t12469: f64, t1737: f64, t43762: f64, t43771: f64, t43773: f64, t43781: f64, t43783: f64, t43785: f64, t43787: f64, t45106: f64, t45107: f64, t56151: f64, t56155: f64, t3362: f64, t462: f64, t51959: f64, t52011: f64, t44348: f64, t44919: f64, t12327: f64, t3391: f64, t5079: f64, t12331: f64, t1134: f64, t16926: f64, t3390: f64, t16857: f64, t3399: f64, t12322: f64, t5071: f64, t3407: f64, t56159: f64, t56163: f64, t56167: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t58005, t58023) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3146(t12469, t1737, t43762, t43771, t43773, t43781, t43783, t43785, t43787, t45106, t45107, t56151, t56155);
        let (t58029, t58032, t58035, t58038, t58041, t58044) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3147(t3362, t462, t51959, t52011, t44348, t44919, t12327, t3391, t5079, t12331, t1134, t16926, t3390);
        let (t58046, t58048, t58051, t58053) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3148(t16857, t3399, t12322, t5071, t1134, t16926, t3407, t56159, t56163, t56167, t58029, t58032, t58035, t58038, t58041, t58044);
    (t58005, t58023, t58029, t58032, t58035, t58038, t58041, t58044, t58046, t58048, t58051, t58053)
}
