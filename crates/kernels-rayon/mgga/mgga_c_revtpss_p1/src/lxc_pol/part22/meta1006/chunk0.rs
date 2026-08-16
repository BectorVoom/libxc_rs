//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3438/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3438(t11524: f64, t19467: f64, t981: f64, t15538: f64, t4719: f64, t15542: f64, t41224: f64, t6189: f64, t19147: f64, t3022: f64, t18900: f64, t3333: f64, t41937: f64, t5023: f64, t6400: f64, t64335: f64, t64338: f64, t64340: f64, t64342: f64, t64344: f64, t64346: f64, t64404: f64, t64465: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t64521 = 0.17315859105681463759e2_f64 * t981 * t19467 * t11524;
    let t64523 = 0.46785788981077169656e1_f64 * t4719 * t15538;
    let t64527 = 0.12304822629859687989e5_f64 * t981 * t41224 * t6189 * t15542;
    let t64529 = 0.23392894490538584828e1_f64 * t3022 * t19147;
    let t64531 = 0.20508037716432813316e4_f64 * t3022 * t18900;
    let t64532 = -6.0_f64 * t3333 * t41937 * t5023 * t6400 + t64335 + t64338 + t64340 + t64342 + t64344 - t64346 - t64404 + t64465 - t64521 + t64523 + t64527 + t64529 - t64531;
    (t64521, t64523, t64527, t64529, t64531, t64532)
}
