//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta298 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1281;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta298(t1379: f64, t9709: f64, t2689: f64, t3952: f64, t1413: f64, t3889: f64, t547: f64, t807: f64, t9646: f64, t2236: f64, t66: f64, t240: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t9711, t9712, t9714, t9716, t9718, t9720, t9721) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1281(t1379, t9709, t2689, t3952, t1413, t3889, t547, t807, t9646, t2236, t66, t240);
    (t9711, t9712, t9714, t9716, t9718, t9720, t9721)
}
