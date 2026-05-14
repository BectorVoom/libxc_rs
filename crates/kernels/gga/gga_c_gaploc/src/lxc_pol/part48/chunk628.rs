//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 628/861 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk628<F: Float>(t11362: F, t3377: F, t13296: F, t189: F, t188: F, t600: F, t568: F, t13322: F, t531: F, t13423: F, t13424: F, t13425: F, t13428: F, t13430: F, t13436: F, t13440: F, t13442: F, t1562: F, t193: F, t557: F, t597: F) -> (F, F, F, F, F, F) {
    let t13444 = 0.10725146985555128001e1 * t11362 * t3377;
    let t13445 = t189 * t13296;
    let t13446 = t188 * t13445;
    let t13449 = t600 * t13296;
    let t13450 = t568 * t13449;
    let t13453 = t531 * t13322;
    let t13456 = -t13423 - t13424 - t13425 - t13428 - 0.13803453343411469884e2 * t1562 * t13430 + t13436 - t13440 - t13442 - t13444 + 0.35750489951850426669e0 * t13446 * t193 + 0.23005755572352449806e1 * t597 * t13450 - 0.35750489951850426669e0 * t557 * t13453;
    (t13445, t13446, t13449, t13450, t13453, t13456)
}
