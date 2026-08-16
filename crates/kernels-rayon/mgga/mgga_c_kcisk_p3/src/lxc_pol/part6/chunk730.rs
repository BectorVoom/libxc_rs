//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 730/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk730(t373: f64, t4128: f64, t357: f64, t4079: f64, t346: f64, t1311: f64, t163: f64, t24: f64, t3951: f64, t398: f64, t963: f64, t13522: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t13565 = 1.0_f64 / t4128 / t373;
    let t13587 = 1.0_f64 / t4079 / t357;
    let t13588 = t346 * t13587;
    let t13603 = t163 * t1311;
    let t13607 = t24 * t3951;
    let t13614 = t963 * t398;
    let t13618 = 28.0_f64 / 27.0_f64 * t13522;
    (t13565, t13588, t13603, t13607, t13614, t13618)
}
