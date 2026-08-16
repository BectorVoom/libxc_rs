//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta267 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1477;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1478;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1479;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1480;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1481;
use chunk5::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1482;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta267(t3869: f64, t9866: f64, t9524: f64, t9542: f64, t9577: f64, t9579: f64, t9581: f64, t9588: f64, t9598: f64, t9854: f64, t9857: f64, t9859: f64, t9862: f64, t9865: f64, t225: f64, t9849: f64, t9850: f64, t9852: f64, t4010: f64, t73: f64, t9400: f64, t3889: f64, t9737: f64, t1394: f64, t9628: f64, t1392: f64, t1395: f64, t4045: f64, t4050: f64, t4053: f64, t539: f64, t541: f64, t5650: f64, t543: f64, t1390: f64, t828: f64, t3926: f64, t3930: f64, t1398: f64, t3923: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t9868, t9869) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1477(t3869, t9866, t9524, t9542, t9577, t9579, t9581, t9588, t9598, t9854, t9857, t9859, t9862, t9865);
        let (t9872, t9881, t9884, t9887) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1478(t225, t9849, t9850, t9852, t9869, t4010, t73, t9400, t3889, t9737, t1394, t9628);
        let t9890 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1479(t1392, t1395, t4045, t4050, t4053, t539, t541, t5650, t9872, t9881, t9884, t9887);
        let t9891 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1480(t543, t9890);
        let (t9893, t9896, t9898) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1481(t1390, t828, t9891, t3926, t3930, t1398, t3923);
        let t9899 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1482(t543, t9898);
    (t9868, t9872, t9881, t9884, t9887, t9890, t9891, t9893, t9896, t9898, t9899)
}
