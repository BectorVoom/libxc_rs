//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta621 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1962;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1963;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta621<F: Float>(t21881: F, t94: F, t1497: F, t4237: F, t77: F, t1493: F, t4241: F, t5872: F, t640: F, t21809: F, t84: F, t4186: F, t2242: F, t5826: F, t19680: F, t603: F, t21663: F, t607: F, t5868: F, t644: F, t13269: F, t1470: F, t4173: F, t4181: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t108714, t108733, t108737, t108745, t108749, t108759) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1962::<F>(t21881, t94, t1497, t4237, t77, t1493, t4241, t5872, t640, t21809, t84, t4186);
        let (t108762, t108765, t108769, t108792, t108807, t108810) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1963::<F>(t2242, t5826, t19680, t603, t21663, t607, t5868, t644, t77, t13269, t1470, t4173, t4181);
    (t108714, t108733, t108737, t108745, t108749, t108759, t108762, t108765, t108769, t108792, t108807, t108810)
}
