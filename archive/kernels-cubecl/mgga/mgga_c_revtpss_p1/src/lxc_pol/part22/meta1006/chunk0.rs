//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3438/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3438<F: Float>(t11524: F, t19467: F, t981: F, t15538: F, t4719: F, t15542: F, t41224: F, t6189: F, t19147: F, t3022: F, t18900: F, t3333: F, t41937: F, t5023: F, t6400: F, t64335: F, t64338: F, t64340: F, t64342: F, t64344: F, t64346: F, t64404: F, t64465: F) -> (F, F, F, F, F, F) {
    let t64521 = F::cast_from(0.17315859105681463759e2_f64) * t981 * t19467 * t11524;
    let t64523 = F::cast_from(0.46785788981077169656e1_f64) * t4719 * t15538;
    let t64527 = F::cast_from(0.12304822629859687989e5_f64) * t981 * t41224 * t6189 * t15542;
    let t64529 = F::cast_from(0.23392894490538584828e1_f64) * t3022 * t19147;
    let t64531 = F::cast_from(0.20508037716432813316e4_f64) * t3022 * t18900;
    let t64532 = -F::cast_from(6.0_f64) * t3333 * t41937 * t5023 * t6400 + t64335 + t64338 + t64340 + t64342 + t64344 - t64346 - t64404 + t64465 - t64521 + t64523 + t64527 + t64529 - t64531;
    (t64521, t64523, t64527, t64529, t64531, t64532)
}
