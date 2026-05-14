//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 879/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk879<F: Float>(t2223: F, t23548: F, t9144: F, t1901: F, t23484: F, t23487: F, t23492: F, t23497: F, t23502: F, t23507: F, t23511: F, t23515: F, t23520: F, t23524: F, t23529: F, t23532: F, t23534: F, t23538: F, t23543: F, t23546: F, t446: F) -> (F, F, F) {
    let t23549 = t23548 * t2223;
    let t23550 = t9144 * t23549;
    let t23553 = 2.0 / 9.0 * t23484 + 2.0 / 9.0 * t446 * t23487 - 2.0 * t446 * t23492 - 2.0 / 3.0 * t446 * t23497 - 2.0 * t446 * t23502 - 2.0 / 3.0 * t446 * t23507 - 4.0 / 9.0 * t1901 * t23511 - 2.0 / 9.0 * t446 * t23515 + 2.0 / 3.0 * t446 * t23520 + 4.0 / 3.0 * t446 * t23524 + 4.0 / 3.0 * t446 * t23529 - 4.0 / 9.0 * t23532 - 2.0 / 9.0 * t23534 + 2.0 / 3.0 * t446 * t23538 + 4.0 / 3.0 * t446 * t23543 + 2.0 / 9.0 * t23546 - 2.0 / 9.0 * t1901 * t23550;
    (t23549, t23550, t23553)
}
