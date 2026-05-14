//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 853/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk853<F: Float>(t13475: F, t689: F, t6023: F, t13469: F, t24311: F, t24389: F, t6: F, t17836: F, t24330: F, t6832: F, t6055: F, t172: F, t6818: F, t6820: F, t6815: F, t6043: F, t6824: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t27557 = t13475 * t689;
    let t27558 = t6023 * t27557;
    let t27561 = t13469 * t689;
    let t27562 = t24311 * t27561;
    let t27565 = t24389 * t6;
    let t27566 = t17836 * t27565;
    let t27569 = t24330 * t6832;
    let t27570 = t6055 * t27569;
    let t27574 = t6818 * t172;
    let t27575 = t27574 * t6820;
    let t27576 = t6815 * t27575;
    let t27579 = t6043 * t24330 * t6824;
    (t27557, t27558, t27561, t27562, t27565, t27566, t27569, t27570, t27574, t27575, t27576, t27579)
}
