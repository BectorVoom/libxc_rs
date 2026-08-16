//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta435 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1944;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1945;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta435(t13944: f64, t1399: f64, t5673: f64, t2689: f64, t5618: f64, t1413: f64, t5591: f64, t547: f64, t807: f64, t5609: f64, t808: f64, t9845: f64, t1885: f64, t9909: f64, t3936: f64, t3938: f64, t13937: f64, t13943: f64, t3934: f64, t9796: f64, t9799: f64, t9804: f64, t9822: f64) -> (f64, f64, f64, f64, f64, f64) {
        let (t13946, t13949, t13951, t13952, t13954, t13955, t13956) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1944(t13944, t1399, t5673, t2689, t5618, t1413, t5591, t547, t807, t5609, t808, t9845);
        let (t13962, t13965) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1945(t1885, t9909, t13944, t3936, t3938, t13937, t13943, t13946, t13949, t13954, t13956, t3934, t9796, t9799, t9804, t9822);
    (t13946, t13951, t13952, t13955, t13962, t13965)
}
