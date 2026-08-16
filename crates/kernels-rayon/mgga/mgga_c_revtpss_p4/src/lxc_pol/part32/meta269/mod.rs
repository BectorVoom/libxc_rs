//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta269 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1132;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1133;
use chunk2::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1134;
use chunk3::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1135;
use chunk4::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1136;
use chunk5::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1137;
use chunk6::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1138;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta269(t5: f64, t1923: f64, t2048: f64, t7343: f64, t7351: f64, t7702: f64, t7706: f64, t7709: f64, t7964: f64, t117: f64, t1843: f64, t2055: f64, t114: f64, t7370: f64, t7738: f64, t508: f64, t1518: f64, t2089: f64, t2071: f64, t7749: f64, t7391: f64, t7393: f64, t7394: f64, t7396: f64, t7753: f64, t7755: f64, t7757: f64, t225: f64, t1579: f64, t2061: f64, t7071: f64, t1558: f64, t231: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t7968, t7969) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1132(t5, t1923, t2048, t7343, t7351, t7702, t7706, t7709, t7964, t117);
        let t7978 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1133(t1843, t2055);
        let t7983 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1134(t114, t7370, t7738);
        let t7984 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1135(t508, t7983);
        let (t7988, t7991, t7997) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1136(t1518, t2089, t2071, t7749, t7391, t7393, t7394, t7396, t7753, t7755, t7757);
        let (t7998, t8006) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1137(t225, t7997, t1579, t2061);
        let (t8007, t8011) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1138(t7071, t8006, t1558, t2061, t231);
    (t7968, t7969, t7978, t7983, t7984, t7988, t7991, t7997, t7998, t8006, t8007, t8011)
}
