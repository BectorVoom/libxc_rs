//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 699/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk699<F: Float>(t14657: F, t14683: F, t14655: F, t14662: F, t14666: F, t14669: F, t14673: F, t14676: F, t14680: F, t14688: F, t14692: F, t14715: F, t10246: F, t10276: F, t10279: F, t10282: F, t10286: F, t10394: F, t10400: F, t14697: F, t14701: F, t14706: F) -> (F, F) {
    let t15089 = 2.0 / 27.0 * t14657;
    let t15096 = 4.0 / 9.0 * t14683;
    let t15099 = 2.0 / 27.0 * t14655 - t15089 + 2.0 / 9.0 * t14662 + t14666 / 9.0 + 4.0 / 9.0 * t14669 - 2.0 / 9.0 * t14673 - 2.0 / 3.0 * t14676 - 4.0 / 9.0 * t14680 - t15096 + 4.0 / 27.0 * t14688 - 4.0 / 9.0 * t14692;
    let t15111 = 4.0 / 81.0 * t14715;
    let t15112 = 4.0 / 3.0 * t14697 + 2.0 / 3.0 * t14701 - 2.0 * t14706 + t10394 / 9.0 - 8.0 / 27.0 * t10400 - 2.0 / 9.0 * t10276 - 2.0 / 27.0 * t10246 - 8.0 / 81.0 * t10279 + t10282 / 27.0 + 2.0 / 81.0 * t10286 - t15111;
    (t15099, t15112)
}
