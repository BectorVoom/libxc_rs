//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta189 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk911;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk912;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta189(t786: f64, t9679: f64, t1359: f64, t9292: f64, t1363: f64, t9288: f64, t1362: f64, t2237: f64, t240: f64, t550: f64, t816: f64, t1379: f64, t547: f64, t9646: f64, t2236: f64, t66: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t9680, t9691, t9692, t9694, t9707, t9711) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk911(t786, t9679, t1359, t9292, t1363, t9288, t1362, t2237, t240, t550, t816, t1379);
        let (t9718, t9720) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk912(t547, t9646, t2236, t66);
    (t9680, t9691, t9692, t9694, t9707, t9711, t9718, t9720)
}
