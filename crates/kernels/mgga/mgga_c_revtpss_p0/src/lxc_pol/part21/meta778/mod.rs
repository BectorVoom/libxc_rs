//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta778 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2770;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2771;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2772;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2773;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta778<F: Float>(t50893: F, t162: F, t40188: F, t14331: F, t40186: F, t40203: F, t40205: F, t14362: F, t9572: F, t37: F, t4391: F, t2612: F, t150: F, t190: F, t50034: F, t40076: F, t40079: F, t40194: F, t40198: F, t50884: F, t50887: F, t50889: F, t50891: F, t50892: F, t10489: F, t10618: F, t10635: F, t10818: F, t14468: F, t14643: F, t14648: F, t14649: F, t14652: F, t1553: F, t1555: F, t225: F, t227: F, t229: F, t2394: F, t2430: F, t2639: F, t4409: F, t4415: F, t4416: F, t50151: F, t50391: F, t50844: F, t50845: F, t50847: F, t50848: F, t50851: F, t50854: F, t50882: F, t775: F, t832: F, t853: F, t10627: F, t10628: F, t10632: F, t14633: F, t14653: F, t14656: F, t14659: F, t18592: F, t231: F, t2634: F, t2642: F, t4417: F, t4420: F, t50396: F, t73: F, t830: F, t833: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t50894, t50897, t50898, t50899, t50900, t50902, t50905) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2770::<F>(t50893, t162, t40188, t14331, t40186, t40203, t40205, t14362, t9572, t37, t4391, t2612);
        let (t50907, t50908) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2771::<F>(t150, t190, t50034, t40076, t40079, t40194, t40198, t50884, t50887, t50889, t50891, t50892, t50894, t50897, t50898, t50899, t50900, t50902, t50905);
        let t50914 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2772::<F>(t10489, t10618, t10635, t10818, t14468, t14643, t14648, t14649, t14652, t1553, t1555, t225, t227, t229, t2394, t2430, t2639, t4409, t4415, t4416, t50151, t50391, t50844, t50845, t50847, t50848, t50851, t50854, t50882, t50908, t775, t832, t853);
        let t50916 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2773::<F>(t10627, t10628, t10632, t14633, t14643, t14653, t14656, t14659, t1553, t18592, t231, t2634, t2642, t4409, t4415, t4417, t4420, t50396, t50914, t73, t830, t833);
    (t50894, t50897, t50898, t50899, t50900, t50902, t50905, t50907, t50916)
}
