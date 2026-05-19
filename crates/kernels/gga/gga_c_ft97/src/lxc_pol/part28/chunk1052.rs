//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1052/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1052<F: Float>(t100776: F, t6426: F, t1701: F, t25684: F, t115567: F, t136517: F, t136520: F, t136555: F, t136566: F, t136684: F, t136772: F, t145303: F, t22590: F, t22623: F, t25631: F, t25680: F, t3034: F, t3038: F, t32301: F, t34424: F, t34444: F, t7889: F, t93169: F) -> (F, F, F) {
    let t145491 = t6426 * t100776;
    let t145501 = t1701 * t25684;
    let t145504 = -F::cast_from(0.22227677429409423704e-2_f64) * t136772 * t34444 - F::cast_from(0.11854761295685025975e-1_f64) * t22623 * t145303 + F::cast_from(0.10338048737805743097e-3_f64) * t136555 * t34424 + F::cast_from(0.10338048737805743097e-3_f64) * t136684 * t34424 - F::cast_from(0.46509801892875584e-1_f64) * t136517 * t25631 + F::cast_from(0.23254900946437792e-1_f64) * t136520 * t3034 + F::cast_from(0.46509801892875584e-2_f64) * t32301 * t3038 + F::cast_from(0.29693535778629056444e-3_f64) * t136566 * t93169 * t145491 + F::cast_from(0.29693535778629056444e-3_f64) * t136566 * t93169 * t115567 + F::cast_from(0.44455354858818847408e-2_f64) * t22590 * t1701 * t25680 - F::cast_from(0.44455354858818847408e-2_f64) * t7889 * t145501;
    (t145491, t145501, t145504)
}
