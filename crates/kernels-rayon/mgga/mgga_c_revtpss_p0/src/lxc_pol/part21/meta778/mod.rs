//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta778 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2770;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2771;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2772;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2773;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta778(t50893: f64, t162: f64, t40188: f64, t14331: f64, t40186: f64, t40203: f64, t40205: f64, t14362: f64, t9572: f64, t37: f64, t4391: f64, t2612: f64, t150: f64, t190: f64, t50034: f64, t40076: f64, t40079: f64, t40194: f64, t40198: f64, t50884: f64, t50887: f64, t50889: f64, t50891: f64, t50892: f64, t10489: f64, t10618: f64, t10635: f64, t10818: f64, t14468: f64, t14643: f64, t14648: f64, t14649: f64, t14652: f64, t1553: f64, t1555: f64, t225: f64, t227: f64, t229: f64, t2394: f64, t2430: f64, t2639: f64, t4409: f64, t4415: f64, t4416: f64, t50151: f64, t50391: f64, t50844: f64, t50845: f64, t50847: f64, t50848: f64, t50851: f64, t50854: f64, t50882: f64, t775: f64, t832: f64, t853: f64, t10627: f64, t10628: f64, t10632: f64, t14633: f64, t14653: f64, t14656: f64, t14659: f64, t18592: f64, t231: f64, t2634: f64, t2642: f64, t4417: f64, t4420: f64, t50396: f64, t73: f64, t830: f64, t833: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t50894, t50897, t50898, t50899, t50900, t50902, t50905) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2770(t50893, t162, t40188, t14331, t40186, t40203, t40205, t14362, t9572, t37, t4391, t2612);
        let (t50907, t50908) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2771(t150, t190, t50034, t40076, t40079, t40194, t40198, t50884, t50887, t50889, t50891, t50892, t50894, t50897, t50898, t50899, t50900, t50902, t50905);
        let t50914 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2772(t10489, t10618, t10635, t10818, t14468, t14643, t14648, t14649, t14652, t1553, t1555, t225, t227, t229, t2394, t2430, t2639, t4409, t4415, t4416, t50151, t50391, t50844, t50845, t50847, t50848, t50851, t50854, t50882, t50908, t775, t832, t853);
        let t50916 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2773(t10627, t10628, t10632, t14633, t14643, t14653, t14656, t14659, t1553, t18592, t231, t2634, t2642, t4409, t4415, t4417, t4420, t50396, t50914, t73, t830, t833);
    (t50894, t50897, t50898, t50899, t50900, t50902, t50905, t50907, t50916)
}
