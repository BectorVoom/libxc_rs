//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta386 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1453;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1454;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1455;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta386<F: Float>(t4057: F, t5673: F, t5674: F, t13848: F, t3938: F, t9818: F, t9816: F, t125: F, t5658: F, t1399: F, t2689: F, t5618: F, t1413: F, t5591: F, t547: F, t807: F, t5609: F, t808: F, t9845: F, t1885: F, t9909: F, t3936: F, t3934: F, t9796: F, t9799: F, t9804: F, t9822: F) -> (F, F, F, F, F, F, F) {
        let (t13937, t13941, t13943, t13944, t13946, t13949) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1453::<F>(t4057, t5673, t5674, t13848, t3938, t9818, t9816, t125, t5658, t1399, t2689, t5618);
        let (t13951, t13954, t13956, t13959, t13962) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1454::<F>(t1413, t5591, t547, t807, t5609, t808, t9845, t1885, t9909, t13944, t3936, t3938);
        let t13965 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1455::<F>(t13937, t13943, t13946, t13949, t13954, t13956, t13959, t13962, t3934, t9796, t9799, t9804, t9822);
    (t13937, t13941, t13944, t13946, t13951, t13962, t13965)
}
