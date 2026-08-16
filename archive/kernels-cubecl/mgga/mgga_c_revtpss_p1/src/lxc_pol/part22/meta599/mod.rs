//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta599 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2485;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2486;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta599<F: Float>(t3022: F, t6219: F, t6223: F, t2986: F, t6205: F, t974: F, t981: F, t4708: F, t4724: F, t3336: F, t6396: F, t6184: F, t964: F, t19021: F, t973: F, t11461: F, t11554: F, t15343: F, t1634: F, t19029: F, t19031: F, t19058: F, t19060: F, t19062: F, t2982: F, t4685: F, t6190: F, t6206: F, t6209: F, t965: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t19143, t19145, t19146, t19147, t19149, t19150, t19152, t19153, t19156) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2485::<F>(t3022, t6219, t6223, t2986, t6205, t974, t981, t4708, t4724, t3336, t6396, t6184, t964);
        let (t19167, t19172) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2486::<F>(t19021, t973, t11461, t11554, t15343, t1634, t19029, t19031, t19058, t19060, t19062, t19156, t2982, t4685, t4708, t6190, t6206, t6209, t965, t974);
    (t19143, t19145, t19146, t19147, t19149, t19150, t19152, t19153, t19156, t19167, t19172)
}
