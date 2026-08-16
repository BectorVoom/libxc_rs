//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta679 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2491;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2492;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta679(t13069: f64, t3704: f64, t12941: f64, t3708: f64, t12948: f64, t13058: f64, t12937: f64, t3172: f64, t3711: f64, t13080: f64, t5384: f64, t1231: f64, t12898: f64, t3651: f64, t3655: f64, t43813: f64, t1256: f64, t12890: f64, t1222: f64, t3693: f64, t697: f64, t13021: f64, t140: f64, t12256: f64, t3698: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t44278, t44280, t44283, t44286, t44289, t44291) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2491(t13069, t3704, t12941, t3708, t12948, t13058, t12937, t3172, t3711, t13080, t5384, t1231, t12898);
        let (t44293, t44307, t44326, t44343, t44346, t44348) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2492(t3651, t3655, t43813, t1256, t12890, t1222, t3693, t697, t13021, t140, t12256, t3698);
    (t44278, t44280, t44283, t44286, t44289, t44291, t44293, t44307, t44326, t44343, t44346, t44348)
}
