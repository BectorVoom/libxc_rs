//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 861/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk861<F: Float>(t17544: F, t1365: F, t670: F, t671: F, t1985: F, t666: F, t226: F, t5903: F, t230: F, t5907: F, t1989: F, t678: F, t17531: F, t17533: F, t17536: F, t17539: F, t17543: F) -> (F, F) {
    let t17545 = 32.0 / 15.0 * t17544;
    let t17548 = 0.22443641344164119597e0 * t670 * t1365 * t671;
    let t17549 = t666 * t1985;
    let t17552 = 16.0 / 3.0 * t226 * t5903;
    let t17553 = t5907 * t230;
    let t17555 = t1989 * t678;
    let t17557 = t17531 + t17533 - t17536 - t17539 + t17543 + t17545 + t17548 + 16.0 * t17549 + t17552 + 16.0 / 3.0 * t17553 + 16.0 * t17555;
    (t17545, t17557)
}
