//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta362 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1722;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1723;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1724;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta362<F: Float>(t1021: F, t3201: F, t362: F, t40: F, t361: F, t351: F, t1058: F, t3231: F, t1054: F, t2434: F, t371: F, t373: F, t367: F, t1020: F, t3230: F, t11924: F, t11927: F, t11930: F, t11933: F, t11938: F, t11941: F, t11944: F, t11947: F, t11952: F, t11954: F, t3120: F, t3208: F, t375: F, t3123: F, t3168: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t11956, t11960, t11961, t11962, t11965, t11967, t11970) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1722::<F>(t1021, t3201, t362, t40, t361, t351, t1058, t3231, t1054, t2434, t371, t373);
        let (t11972, t11973, t11976) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1723::<F>(t11970, t367, t1020, t3230, t11924, t11927, t11930, t11933, t11938, t11941, t11944, t11947, t11952, t11954, t11956, t11962, t11965, t11967, t3120, t3208, t375);
        let t11977 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1724::<F>(t3123, t3168);
    (t11956, t11960, t11961, t11962, t11965, t11967, t11970, t11972, t11973, t11976, t11977)
}
