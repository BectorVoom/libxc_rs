//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta274 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1129;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1130;
use chunk2::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1131;
use chunk3::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1132;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta274<F: Float>(t5: F, t114: F, t1923: F, t2048: F, t7343: F, t7351: F, t7702: F, t7706: F, t7709: F, t7964: F, t117: F, t1843: F, t2055: F, t7370: F, t7738: F, t508: F, t1518: F, t2089: F, t2071: F, t7749: F, t7391: F, t7393: F, t7394: F, t7396: F, t7753: F, t7755: F, t7757: F, t225: F, t1579: F, t2061: F, t7071: F, t1558: F, t231: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t7968, t7969, t7978, t7983) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1129::<F>(t5, t114, t1923, t2048, t7343, t7351, t7702, t7706, t7709, t7964, t117, t1843, t2055, t7370, t7738);
        let (t7984, t7988, t7991, t7997) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1130::<F>(t508, t7983, t1518, t2089, t2071, t7749, t7391, t7393, t7394, t7396, t7753, t7755, t7757);
        let (t7998, t8006) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1131::<F>(t225, t7997, t1579, t2061);
        let (t8007, t8011) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1132::<F>(t7071, t8006, t1558, t2061, t231);
    (t7968, t7969, t7978, t7983, t7984, t7988, t7991, t7997, t7998, t8006, t8007, t8011)
}
