//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta386 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1453;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1454;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1455;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta386(t4057: f64, t5673: f64, t5674: f64, t13848: f64, t3938: f64, t9818: f64, t9816: f64, t125: f64, t5658: f64, t1399: f64, t2689: f64, t5618: f64, t1413: f64, t5591: f64, t547: f64, t807: f64, t5609: f64, t808: f64, t9845: f64, t1885: f64, t9909: f64, t3936: f64, t3934: f64, t9796: f64, t9799: f64, t9804: f64, t9822: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t13937, t13941, t13943, t13944, t13946, t13949) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1453(t4057, t5673, t5674, t13848, t3938, t9818, t9816, t125, t5658, t1399, t2689, t5618);
        let (t13951, t13954, t13956, t13959, t13962) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1454(t1413, t5591, t547, t807, t5609, t808, t9845, t1885, t9909, t13944, t3936, t3938);
        let t13965 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1455(t13937, t13943, t13946, t13949, t13954, t13956, t13959, t13962, t3934, t9796, t9799, t9804, t9822);
    (t13937, t13941, t13944, t13946, t13951, t13962, t13965)
}
