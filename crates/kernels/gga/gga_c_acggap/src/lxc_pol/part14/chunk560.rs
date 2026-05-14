//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 560/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk560<F: Float>(t418: F, t4275: F, t4279: F, t4280: F, t4285: F, t4288: F, t4308: F, t4310: F, t4312: F, t4320: F, t4322: F, t4324: F, t4328: F, t4339: F, t5529: F, t5534: F, t5539: F, t5542: F, t5546: F, t5551: F, t5554: F) -> (F,) {
    let t5558 = 0.25724410870841842184e-2 * t418 * t5529 - 0.51448821741683684368e-2 * t418 * t5534 + 0.25724410870841842184e-2 * t418 * t5539 + 0.34299214494455789578e-2 * t5542 + 0.34299214494455789578e-2 * t418 * t5546 + 0.34299214494455789578e-2 * t418 * t5551 - 0.17149607247227894789e-2 * t5554 + t4275 - t4279 - 0.80031500487063509015e-2 * t4280 - 0.85748036236139473945e-2 * t4285 - t4288 + t4308 - t4310 + t4312 - t4320 + t4322 - t4324 + t4328 - t4339;
    (t5558,)
}
