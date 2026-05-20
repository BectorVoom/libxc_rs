//! MGGA_C_REVTPSS lxc pol kernel — _part39_v4rho3tau_2 meta406 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1480;
use chunk1::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1481;
use chunk2::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1482;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_meta406<F: Float>(t1504: F, t665: F, t8268: F, t31054: F, t658: F, t31058: F, t10199: F, t655: F, t2: F, t31026: F, t31028: F, t31030: F, t31033: F, t31035: F, t31259: F, t31261: F, t31264: F, t31268: F, t31271: F, t31274: F, t8258: F, t8267: F, t114: F, t569: F, t2178: F, t5517: F, t1312: F, t13426: F, t18227: F, t2179: F, t2181: F, t2322: F, t27123: F, t28219: F, t31248: F, t4248: F, t4254: F, t5523: F, t651: F, t7732: F, t7889: F, t8274: F, t8278: F, t8280: F, t8353: F, t8367: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t31276, t31277, t31280, t31283, t31284, t31287, t31288, t31291) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1480::<F>(t1504, t665, t8268, t31054, t658, t31058, t10199, t655, t2, t31026, t31028, t31030, t31033, t31035, t31259, t31261, t31264, t31268, t31271, t31274, t8258, t8267);
        let t31292 = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1481::<F>(t114, t31291);
        let (t31293, t31299, t31303) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1482::<F>(t31292, t569, t2178, t5517, t1312, t13426, t18227, t2179, t2181, t2322, t27123, t28219, t31248, t4248, t4254, t5523, t651, t7732, t7889, t8274, t8278, t8280, t8353, t8367);
    (t31276, t31277, t31280, t31283, t31284, t31287, t31288, t31292, t31293, t31299, t31303)
}
