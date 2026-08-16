//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1695/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1695(t1668: f64, t24031: f64, t88004: f64, t88007: f64, t88012: f64, t88016: f64, t88023: f64, t88026: f64, t88028: f64, t88030: f64, t88034: f64, t88036: f64, t88038: f64) -> (f64, f64) {
    let t88948 = t24031 * t1668;
    let t88980 = -t88004 + t88007 - t88012 + t88016 - t88023 + t88026 - t88028 - t88030 + t88034 - t88036 - t88038;
    (t88948, t88980)
}
