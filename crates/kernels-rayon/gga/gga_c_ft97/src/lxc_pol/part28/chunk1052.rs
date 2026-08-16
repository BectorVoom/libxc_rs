//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1052/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1052(t100776: f64, t6426: f64, t1701: f64, t25684: f64, t115567: f64, t136517: f64, t136520: f64, t136555: f64, t136566: f64, t136684: f64, t136772: f64, t145303: f64, t22590: f64, t22623: f64, t25631: f64, t25680: f64, t3034: f64, t3038: f64, t32301: f64, t34424: f64, t34444: f64, t7889: f64, t93169: f64) -> (f64, f64, f64) {
    let t145491 = t6426 * t100776;
    let t145501 = t1701 * t25684;
    let t145504 = -0.22227677429409423704e-2_f64 * t136772 * t34444 - 0.11854761295685025975e-1_f64 * t22623 * t145303 + 0.10338048737805743097e-3_f64 * t136555 * t34424 + 0.10338048737805743097e-3_f64 * t136684 * t34424 - 0.46509801892875584e-1_f64 * t136517 * t25631 + 0.23254900946437792e-1_f64 * t136520 * t3034 + 0.46509801892875584e-2_f64 * t32301 * t3038 + 0.29693535778629056444e-3_f64 * t136566 * t93169 * t145491 + 0.29693535778629056444e-3_f64 * t136566 * t93169 * t115567 + 0.44455354858818847408e-2_f64 * t22590 * t1701 * t25680 - 0.44455354858818847408e-2_f64 * t7889 * t145501;
    (t145491, t145501, t145504)
}
