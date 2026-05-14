//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 991/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk991<F: Float>(t26435: F, t3188: F, t11472: F, t1332: F, t1557: F, t11556: F, t379: F, t6538: F, t8557: F, t376: F, t6526: F, t89: F, t1307: F, t3291: F, t452: F, t1901: F, t23239: F, t23263: F, t23283: F, t26412: F, t26416: F, t26420: F, t26425: F, t26428: F, t26432: F, t446: F) -> (F, F, F, F, F, F, F, F, F) {
    let t26436 = t26435 * t3188;
    let t26437 = t11472 * t26436;
    let t26440 = t1332 * t1557;
    let t26441 = t26440 * t3188;
    let t26442 = t11556 * t26441;
    let t26445 = t6538 * t379;
    let t26446 = t8557 * t26445;
    let t26451 = t89 * t376 * t6526;
    let t26454 = t452 * t3291 * t1307;
    let t26457 = -t23239 / 27.0 + t446 * t26412 / 3.0 + t446 * t26416 / 3.0 + t446 * t26420 / 3.0 + t446 * t26425 / 3.0 + t26428 / 27.0 - 2.0 / 9.0 * t23263 - t446 * t26432 / 3.0 - 2.0 / 9.0 * t1901 * t26437 + 2.0 / 27.0 * t1901 * t26442 - t1901 * t26446 / 9.0 + t23283 / 9.0 - t26451 / 9.0 - t446 * t26454 / 3.0;
    (t26436, t26437, t26440, t26441, t26442, t26445, t26446, t26454, t26457)
}
