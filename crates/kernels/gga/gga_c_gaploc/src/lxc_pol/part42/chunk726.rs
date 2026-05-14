//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 726/880 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk726<F: Float>(t39624: F, t39626: F, t39632: F, t39646: F, t39648: F, t39650: F, t2268: F, t35901: F, t894: F, t426: F, t44386: F, t535: F, t13258: F, t484: F, t11481: F, t2321: F, t882: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t44595 = 7.0 / 256.0 * t39624;
    let t44596 = 63.0 / 8192.0 * t39626;
    let t44597 = 63.0 / 524288.0 * t39632;
    let t44598 = 21.0 / 524288.0 * t39646;
    let t44599 = 21.0 / 8192.0 * t39648;
    let t44600 = 7.0 / 768.0 * t39650;
    let t44618 = 0.56910013271352299198e-1 * t2268 * t894 * t35901;
    let t44622 = 0.28455006635676149599e-1 * t2268 * t535 * t44386 * t426;
    let t44623 = t484 * t13258;
    let t44624 = 0.15808337019820083111e-2 * t44623;
    let t44626 = t882 * t11481 * t2321;
    (t44595, t44596, t44597, t44598, t44599, t44600, t44618, t44622, t44624, t44626)
}
