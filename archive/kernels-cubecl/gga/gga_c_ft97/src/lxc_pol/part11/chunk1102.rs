//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1102/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1102<F: Float>(t10912: F, t2253: F, t3628: F, t895: F, t10904: F, t230: F, t2440: F, t900: F, t10864: F, t10894: F, t10905: F, t14487: F, t18862: F, t2265: F, t2409: F, t2923: F, t2938: F, t2939: F, t2951: F, t41454: F, t41464: F, t41473: F, t41482: F, t41490: F, t4334: F, t4342: F, t505: F, t631: F, t684: F, t898: F, t904: F, t9572: F, t9587: F, t9596: F) -> F {
    let t43094 = t2253 * t10912;
    let t43101 = t3628 * t895;
    let t43109 = t230 * t10904;
    let t43122 = t2440 * t900;
    let t43140 = F::cast_from(12.0_f64) * t2265 * t18862 * t505 * t2951 * t904 + F::cast_from(12.0_f64) * t43094 - F::cast_from(6.0_f64) * t631 * t898 * t2938 * t10894 * t904 - F::cast_from(160.0_f64) / F::cast_from(81.0_f64) * t43101 + F::cast_from(6.0_f64) * t2265 * t4342 * t41482 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t2265 * t4342 * t41490 - F::cast_from(16.0_f64) * t2265 * t43109 * t684 * t10905 - F::cast_from(12.0_f64) * t2265 * t10864 * t2409 * t2939 - F::cast_from(8.0_f64) * t2265 * t2923 * t9587 * t904 - F::cast_from(16.0_f64) / F::cast_from(27.0_f64) * t2265 * t43122 * t9572 * t904 - F::cast_from(2.0_f64) * t2265 * t4334 * t41464 + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t2265 * t4334 * t41473 + F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t2265 * t14487 * t41454 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t2265 * t2923 * t9596 * t904;
    t43140
}
