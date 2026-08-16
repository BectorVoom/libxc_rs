//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3923/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3923(t22466: f64, t3889: f64, t40067: f64, t40072: f64, t4139: f64, t47109: f64, t47116: f64, t47118: f64, t6816: f64, t74134: f64, t74135: f64, t74136: f64, t74137: f64, t74138: f64, t9599: f64) -> f64 {
    let t75408 = -3.0_f64 * t22466 * t3889 * t4139 - 3.0_f64 * t4139 * t6816 * t9599 + t40067 - t40072 - t47109 + t47116 - t47118 + t74134 - t74135 - t74136 + t74137 - t74138;
    t75408
}
