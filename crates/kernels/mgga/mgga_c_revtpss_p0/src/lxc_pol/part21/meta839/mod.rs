//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta839 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3146;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3147;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3148;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta839<F: Float>(t12469: F, t1737: F, t43762: F, t43771: F, t43773: F, t43781: F, t43783: F, t43785: F, t43787: F, t45106: F, t45107: F, t56151: F, t56155: F, t3362: F, t462: F, t51959: F, t52011: F, t44348: F, t44919: F, t12327: F, t3391: F, t5079: F, t12331: F, t1134: F, t16926: F, t3390: F, t16857: F, t3399: F, t12322: F, t5071: F, t3407: F, t56159: F, t56163: F, t56167: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t58005, t58023) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3146::<F>(t12469, t1737, t43762, t43771, t43773, t43781, t43783, t43785, t43787, t45106, t45107, t56151, t56155);
        let (t58029, t58032, t58035, t58038, t58041, t58044) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3147::<F>(t3362, t462, t51959, t52011, t44348, t44919, t12327, t3391, t5079, t12331, t1134, t16926, t3390);
        let (t58046, t58048, t58051, t58053) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3148::<F>(t16857, t3399, t12322, t5071, t1134, t16926, t3407, t56159, t56163, t56167, t58029, t58032, t58035, t58038, t58041, t58044);
    (t58005, t58023, t58029, t58032, t58035, t58038, t58041, t58044, t58046, t58048, t58051, t58053)
}
