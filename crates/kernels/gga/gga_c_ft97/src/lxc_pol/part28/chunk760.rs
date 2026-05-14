//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 760/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk760<F: Float>(t34511: F, t488: F, t10969: F, t7274: F, t32066: F, t32093: F, t34373: F, t34377: F, t34382: F, t34387: F, t34391: F, t34395: F, t34399: F, t34403: F, t34408: F, t32114: F, t32332: F, t32349: F, t34413: F, t34418: F, t34485: F, t34489: F, t34493: F, t34497: F, t34501: F, t34505: F, t34509: F) -> (F, F, F, F) {
    let t34512 = t488 * t34511;
    let t34514 = t10969 * t7274;
    let t34524 = t34373 / 2.0 + t32066 + 2.0 / 9.0 * t34377 + 4.0 / 3.0 * t34382 - 2.0 / 3.0 * t34387 - t34391 / 6.0 - t32093 - t34395 / 9.0 - t34399 + 2.0 / 3.0 * t34403 + t34408 / 12.0;
    let t34534 = t32114 + t34413 / 18.0 + t34418 / 3.0 - t34485 / 6.0 - t32332 - 2.0 / 9.0 * t34489 - 2.0 * t34493 + 4.0 / 3.0 * t34497 + t32349 + t34501 / 9.0 + 2.0 / 3.0 * t34505 - t34509 / 3.0;
    (t34512, t34514, t34524, t34534)
}
