//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 692/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk692<F: Float>(t32111: F, t32364: F, t103: F, t1337: F, t5618: F, t28: F, t497: F, t7212: F, t32325: F, t369: F, t108: F, t432: F, t5507: F, t1286: F, t1310: F, t31997: F, t32000: F, t32002: F, t32013: F, t32016: F, t32021: F, t32025: F, t32054: F, t5495: F, t5501: F, t5504: F, t5620: F, t5624: F, t7162: F, t7168: F, t7214: F, t7218: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t32365 = t32111 + t32364;
    let t32366 = t32365 * t103;
    let t32370 = t5618 * t1337;
    let t32371 = t28 * t32370;
    let t32374 = t7212 * t497;
    let t32375 = t28 * t32374;
    let t32378 = t369 * t32325;
    let t32379 = t32378 * t108;
    let t32380 = t28 * t32379;
    let t32385 = t1337 * t432;
    let t32386 = t5507 * t32385;
    let t32387 = t28 * t32386;
    let t32390 = -t31997 - t32000 - t1286 * t32002 / 3.0 + t5495 * t7214 / 6.0 + t7162 * t5624 / 6.0 + t7162 * t5620 / 6.0 - t5501 * t32013 / 18.0 - t32016 * t5504 / 18.0 + t5501 * t32021 / 9.0 - t32025 + t32054 * t1310 / 6.0 + 2.0 * t32366 + t5495 * t7218 / 3.0 + t1286 * t32371 / 3.0 + t1286 * t32375 / 6.0 + t1286 * t32380 / 6.0 - t5495 * t7168 / 3.0 - 2.0 / 3.0 * t1286 * t32387;
    (t32365, t32366, t32370, t32371, t32374, t32375, t32378, t32379, t32380, t32385, t32386, t32387, t32390)
}
