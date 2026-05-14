//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1033/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1033<F: Float>(t1196: F, t213: F, t231: F, t6819: F, t1208: F, t6045: F, t1197: F, t6979: F, t1091: F, t28561: F, t28598: F, t6035: F, t1201: F, t1209: F, t1408: F, t19039: F, t25049: F, t25070: F, t25077: F, t28652: F, t28660: F, t28677: F, t28680: F, t30696: F, t30776: F, t30786: F, t30790: F, t30843: F, t5232: F, t5265: F, t6256: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t31409 = t1196 * t213;
    let t31410 = t231 * t31409;
    let t31411 = t6819 * t31410;
    let t31414 = t213 * t1208;
    let t31415 = t231 * t31414;
    let t31416 = t6819 * t31415;
    let t31419 = t1196 * t1208;
    let t31420 = t231 * t31419;
    let t31421 = t6045 * t31420;
    let t31440 = t6979 * t1197;
    let t31446 = t28561 * t1091;
    let t31450 = t28598 * t1091;
    let t31451 = t6035 * t31450;
    let t31454 = -0.4833552354146973393e0 * t28652 * t31411 - 0.4833552354146973393e0 * t28680 * t31416 + 0.40006800655555555556e0 * t25049 * t31421 + 0.4833552354146973393e0 * t28677 * t31411 + 0.4833552354146973393e0 * t28660 * t31416 - 0.22226000364197530865e-1 * t6256 * t30786 - 0.16669500273148148149e-1 * t6256 * t30790 + 0.88904001456790123461e-1 * t6256 * t30843 + 0.33339000546296296298e-1 * t6256 * t30776 + 0.76518236253115177207e1 * t1201 * t30696 - 0.45306850413028723348e0 * t5232 * t1408 - 0.10947790369858991998e1 * t19039 * t31440 + 0.54738951849294959988e0 * t5265 * t6979 * t1209 + 0.66678001092592592595e-1 * t25077 * t6035 * t31446 - 0.66678001092592592595e-1 * t25070 * t31451;
    (t31409, t31410, t31411, t31414, t31415, t31419, t31420, t31421, t31440, t31446, t31450, t31451, t31454)
}
