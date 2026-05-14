//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 768/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk768<F: Float>(t10503: F, t19465: F, t2881: F, t18514: F, t4140: F, t18497: F, t4265: F, t11593: F, t15180: F, t15190: F, t15206: F, t1901: F, t19432: F, t19437: F, t19442: F, t19446: F, t19449: F, t19451: F, t19453: F, t19457: F, t19462: F, t446: F) -> (F, F, F, F) {
    let t19466 = t10503 * t19465;
    let t19467 = t2881 * t19466;
    let t19470 = t4140 * t18514;
    let t19471 = t2881 * t19470;
    let t19474 = t4265 * t18497;
    let t19475 = t2881 * t19474;
    let t19478 = -2.0 * t446 * t19432 + 4.0 / 3.0 * t446 * t19437 - 4.0 / 27.0 * t15180 - t446 * t19442 / 3.0 + 2.0 / 3.0 * t446 * t19446 - t15190 + 2.0 / 9.0 * t19449 + t19451 / 9.0 + t19453 / 9.0 + t15206 - 4.0 / 9.0 * t11593 * t19457 - 2.0 / 9.0 * t1901 * t19462 - 2.0 / 9.0 * t1901 * t19467 - 2.0 / 3.0 * t1901 * t19471 - 8.0 / 9.0 * t11593 * t19475;
    (t19466, t19470, t19474, t19478)
}
