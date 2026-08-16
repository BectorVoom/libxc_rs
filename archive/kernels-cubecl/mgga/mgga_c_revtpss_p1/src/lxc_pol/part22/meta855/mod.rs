//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta855 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2999;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3000;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta855<F: Float>(t2829: F, t4321: F, t689: F, t15054: F, t786: F, t789: F, t2465: F, t4480: F, t9288: F, t1569: F, t2769: F, t10997: F, t10985: F, t15017: F, t15045: F, t2435: F, t15048: F, t2471: F, t15008: F, t10996: F, t14990: F, t41070: F, t14939: F, t212: F, t780: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t50198, t50201, t50205, t50208, t50209) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2999::<F>(t2829, t4321, t689, t15054, t786, t789, t2465, t4480, t9288, t1569, t2769, t10997);
        let (t50214, t50218, t50220, t50222, t50227, t50232) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3000::<F>(t10985, t15017, t15045, t2435, t15048, t2471, t15008, t10996, t14990, t41070, t14939, t212, t689, t780);
    (t50198, t50201, t50205, t50208, t50209, t50214, t50218, t50220, t50222, t50227, t50232)
}
