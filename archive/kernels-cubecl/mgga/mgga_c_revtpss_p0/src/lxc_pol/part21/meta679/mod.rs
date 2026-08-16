//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta679 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2491;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2492;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta679<F: Float>(t13069: F, t3704: F, t12941: F, t3708: F, t12948: F, t13058: F, t12937: F, t3172: F, t3711: F, t13080: F, t5384: F, t1231: F, t12898: F, t3651: F, t3655: F, t43813: F, t1256: F, t12890: F, t1222: F, t3693: F, t697: F, t13021: F, t140: F, t12256: F, t3698: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t44278, t44280, t44283, t44286, t44289, t44291) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2491::<F>(t13069, t3704, t12941, t3708, t12948, t13058, t12937, t3172, t3711, t13080, t5384, t1231, t12898);
        let (t44293, t44307, t44326, t44343, t44346, t44348) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2492::<F>(t3651, t3655, t43813, t1256, t12890, t1222, t3693, t697, t13021, t140, t12256, t3698);
    (t44278, t44280, t44283, t44286, t44289, t44291, t44293, t44307, t44326, t44343, t44346, t44348)
}
