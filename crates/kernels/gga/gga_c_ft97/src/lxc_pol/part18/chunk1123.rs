//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1123/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1123<F: Float>(t23711: F, t93325: F, t23701: F, t22572: F, t23715: F, t23717: F, t23705: F, t23707: F, t8811: F, t93178: F, t135: F, t5555: F, t5824: F, t92574: F, t3392: F, t39801: F, t6: F, t8: F) -> (F, F, F, F, F, F, F, F) {
    let t94854 = t23711 * t93325;
    let t94856 = t23701 * t93325;
    let t94873 = t23715 * t22572 * t23717;
    let t94876 = t23705 * t22572 * t23707;
    let t94891 = t8811 * t93178;
    let t94892 = t5555 * t135;
    let t94932 = t5824 * t92574;
    let t94936 = t3392 * t39801 * t6 * t8;
    (t94854, t94856, t94873, t94876, t94891, t94892, t94932, t94936)
}
