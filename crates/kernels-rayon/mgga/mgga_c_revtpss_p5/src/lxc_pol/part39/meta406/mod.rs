//! MGGA_C_REVTPSS lxc pol kernel — _part39_v4rho3tau_2 meta406 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1480;
use chunk1::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1481;
use chunk2::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1482;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_meta406(t1504: f64, t665: f64, t8268: f64, t31054: f64, t658: f64, t31058: f64, t10199: f64, t655: f64, t2: f64, t31026: f64, t31028: f64, t31030: f64, t31033: f64, t31035: f64, t31259: f64, t31261: f64, t31264: f64, t31268: f64, t31271: f64, t31274: f64, t8258: f64, t8267: f64, t114: f64, t569: f64, t2178: f64, t5517: f64, t1312: f64, t13426: f64, t18227: f64, t2179: f64, t2181: f64, t2322: f64, t27123: f64, t28219: f64, t31248: f64, t4248: f64, t4254: f64, t5523: f64, t651: f64, t7732: f64, t7889: f64, t8274: f64, t8278: f64, t8280: f64, t8353: f64, t8367: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t31276, t31277, t31280, t31283, t31284, t31287, t31288, t31291) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1480(t1504, t665, t8268, t31054, t658, t31058, t10199, t655, t2, t31026, t31028, t31030, t31033, t31035, t31259, t31261, t31264, t31268, t31271, t31274, t8258, t8267);
        let t31292 = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1481(t114, t31291);
        let (t31293, t31299, t31303) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1482(t31292, t569, t2178, t5517, t1312, t13426, t18227, t2179, t2181, t2322, t27123, t28219, t31248, t4248, t4254, t5523, t651, t7732, t7889, t8274, t8278, t8280, t8353, t8367);
    (t31276, t31277, t31280, t31283, t31284, t31287, t31288, t31292, t31293, t31299, t31303)
}
