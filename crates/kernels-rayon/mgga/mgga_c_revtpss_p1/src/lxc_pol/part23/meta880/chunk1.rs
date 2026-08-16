//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2789/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2789(t14141: f64, t14143: f64, t5658: f64, t676: f64, t22252: f64, t555: f64, t1419: f64, t6843: f64, t14224: f64, t14238: f64, t2782: f64, t6861: f64) -> (f64, f64, f64, f64, f64) {
    let t74949 = t14141 * t14143 * t676 * t5658;
    let t74965 = t555 * t22252;
    let t74973 = t1419 * t6843;
    let t74979 = t2782 * t14238 * t14224;
    let t74982 = t1419 * t6861;
    (t74949, t74965, t74973, t74979, t74982)
}
