//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1213/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1213<F: Float>(t29996: F, t8392: F, t26061: F, t3255: F, t29803: F, t102776: F, t103073: F, t103695: F, t11593: F, t116124: F, t11906: F, t16012: F, t16016: F, t16140: F, t16146: F, t16246: F, t16485: F, t1871: F, t1901: F, t23323: F, t25990: F, t26210: F, t26349: F, t3103: F, t446: F, t452: F, t4612: F, t47659: F, t488: F, t5722: F, t6557: F, t83: F, t91539: F, t93636: F, t986: F) -> (F, F) {
    let t117996 = t8392 * t29996;
    let t117998 = t26061 * t3255;
    let t118039 = t8392 * t29803;
    let t118041 = 2.0 / 27.0 * t117996 - 2.0 / 3.0 * t446 * t83 * t117998 + 2.0 / 9.0 * t1901 * t93636 * t4612 - 4.0 / 3.0 * t1901 * t102776 * t16146 - t103695 + 2.0 / 3.0 * t446 * t452 * t488 * t6557 * t3103 - 4.0 / 3.0 * t1901 * t103073 * t16140 + 4.0 / 9.0 * t11593 * t11906 * t26210 - 2.0 * t446 * t83 * t116124 + 4.0 / 9.0 * t47659 * t91539 * t16485 + t446 * t452 * t16246 * t5722 / 3.0 + 4.0 / 3.0 * t446 * t1871 * t986 * t25990 + 2.0 / 9.0 * t1901 * t23323 * t16012 - 2.0 / 27.0 * t1901 * t26349 * t16016 - t118039 / 27.0;
    (t117998, t118041)
}
