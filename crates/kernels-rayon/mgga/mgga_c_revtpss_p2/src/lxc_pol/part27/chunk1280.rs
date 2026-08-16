//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1280/1333 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1280(t786: f64, t94878: f64, t7286: f64, t4132: f64, t689: f64, t7242: f64, t2023: f64, t4075: f64, t9682: f64, t26050: f64, t26072: f64, t213: f64, t26034: f64) -> (f64, f64, f64, f64, f64) {
    let t94894 = t786 * t94878;
    let t94895 = t94894 * t7286;
    let t94898 = t689 * t7242 * t4132;
    let t94901 = t786 * t2023 * t4075;
    let t94902 = t94901 * t9682;
    let t94904 = t26072 * t26050;
    let t94906 = t213 * t26034;
    (t94895, t94898, t94902, t94904, t94906)
}
