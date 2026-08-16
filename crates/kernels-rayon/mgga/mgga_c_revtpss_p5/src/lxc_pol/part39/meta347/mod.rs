//! MGGA_C_REVTPSS lxc pol kernel — _part39_v4rho3tau_2 meta347 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1169;
use chunk1::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1170;
use chunk2::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1171;
use chunk3::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1172;
use chunk4::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1173;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_meta347(t9858: f64, t9861: f64, t2619: f64, t5635: f64, t13664: f64, t13667: f64, t13669: f64, t13671: f64, t13673: f64, t13682: f64, t13683: f64, t9524: f64, t9542: f64, t9588: f64, t9854: f64, t9865: f64, t9868: f64, t13881: f64, t13882: f64, t13884: f64, t225: f64, t1392: f64, t73: f64, t13768: f64, t3829: f64, t1412: f64, t5591: f64, t1353: f64, t3889: f64, t5651: f64, t13716: f64, t1394: f64, t1395: f64, t1877: f64, t1879: f64, t4045: f64, t4050: f64, t4053: f64, t539: f64, t541: f64, t5644: f64, t5650: f64, t5652: f64, t5655: f64, t543: f64, t1390: f64, t828: f64, t1398: f64, t1882: f64, t3938: f64, t13789: f64, t13869: f64, t13874: f64, t1388: f64, t13880: f64, t1410: f64, t3934: f64, t9753: f64, t9762: f64, t9766: f64, t9771: f64, t9776: f64, t9780: f64, t9786: f64, t9791: f64, t4057: f64, t5673: f64, t5674: f64, t13848: f64, t9818: f64, t9816: f64, t125: f64, t5658: f64, t1399: f64, t2689: f64, t5618: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t13885, t13886, t13888, t13889) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1169(t9858, t9861, t2619, t5635, t13664, t13667, t13669, t13671, t13673, t13682, t13683, t9524, t9542, t9588, t9854, t9865, t9868);
        let (t13892, t13902, t13907, t13911) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1170(t13881, t13882, t13884, t13889, t225, t1392, t73, t13768, t3829, t1412, t5591, t1353);
        let t13920 = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1171(t3889, t5651, t13716, t1394, t13892, t13902, t13907, t13911, t1392, t1395, t1877, t1879, t4045, t4050, t4053, t539, t541, t5644, t5650, t5652, t5655);
        let (t13921, t13926, t13931) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1172(t13920, t543, t1390, t828, t1398, t1882, t3938, t13789, t13869, t13874, t1388, t13880, t1410, t3934, t9753, t9762, t9766, t9771, t9776, t9780, t9786, t9791);
        let (t13937, t13943, t13944, t13946, t13949) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1173(t4057, t5673, t5674, t13848, t3938, t9818, t9816, t125, t5658, t1399, t2689, t5618);
    (t13885, t13886, t13888, t13921, t13926, t13931, t13937, t13943, t13944, t13946, t13949)
}
