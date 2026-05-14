//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1358/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1358<F: Float>(t122796: F, t7009: F, t108685: F, t6249: F, t7012: F, t213: F, t231: F, t4088: F, t6819: F, t14763: F, t7005: F, t24330: F, t31381: F, t111881: F, t111889: F, t111892: F, t111895: F, t111901: F, t111908: F, t1196: F, t25049: F, t27506: F, t28579: F, t28583: F, t28652: F, t28677: F, t31398: F, t31411: F, t4125: F, t6045: F, t6242: F) -> (F,) {
    let t127096 = t7009 * t122796;
    let t127099 = t6249 * t108685 * t7012;
    let t127108 = t6819 * t231 * t4088 * t213;
    let t127111 = t14763 * t7005;
    let t127117 = t6249 * t24330 * t31381;
    let t127124 = 0.22226000364197530865e-1 * t111881 - 0.66678001092592592595e-1 * t111889 - 0.66678001092592592595e-1 * t111892 - 0.4445200072839506173e-1 * t111895 + 0.66678001092592592595e-1 * t111901 + 0.53706137268299704368e-1 * t111908 + 0.46992870109762241323e0 * t127096 - 0.17780800291358024692e0 * t127099 + 0.53342400874074074075e0 * t28579 * t31398 + 0.53342400874074074075e0 * t6242 * t27506 * t28583 + 0.4833552354146973393e0 * t28677 * t127108 - 0.4833552354146973393e0 * t127111 * t31411 - 0.4833552354146973393e0 * t28652 * t127108 + 0.33339000546296296297e-1 * t127117 + 0.40006800655555555556e0 * t25049 * t6045 * t231 * t1196 * t4125;
    (t127124,)
}
