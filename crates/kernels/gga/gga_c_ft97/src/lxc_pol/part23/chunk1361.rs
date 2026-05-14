//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1361/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1361<F: Float>(t123233: F, t6256: F, t27659: F, t28666: F, t35455: F, t5295: F, t703: F, t28654: F, t30727: F, t213: F, t231: F, t4125: F, t6819: F, t108817: F, t111935: F, t111953: F, t111956: F, t111959: F, t111968: F, t111989: F, t112060: F, t112071: F, t123713: F, t25077: F, t27506: F, t28552: F, t28561: F, t28587: F, t28660: F, t28677: F, t31421: F, t31451: F, t3746: F, t6035: F, t6249: F, t684: F) -> (F, F, F, F) {
    let t127174 = t6256 * t123233;
    let t127185 = t27659 * t35455 * t28666;
    let t127194 = t703 * t5295;
    let t127199 = t30727 * t28654;
    let t127204 = t6819 * t231 * t213 * t4125;
    let t127209 = -0.55565000910493827163e-2 * t127174 - 0.13335600218518518519e0 * t25077 * t108817 * t28561 * t3746 - 0.66678001092592592595e-1 * t111989 * t31451 + 0.4445200072839506173e-1 * t111935 + t111953 - t111956 - 0.53706137268299704368e-1 * t111959 - 0.9667104708293946786e0 * t112071 * t127185 + 0.8890400145679012346e-1 * t28552 * t123713 - 0.53342400874074074075e0 * t6249 * t27506 * t28587 + 0.1611184118048991131e0 * t111968 + 0.33339000546296296297e-1 * t25077 * t6035 * t127194 * t684 - 0.28195722065857344794e1 * t28677 * t127199 + 0.4833552354146973393e0 * t28660 * t127204 + 0.40006800655555555556e0 * t112060 * t31421;
    (t127185, t127199, t127204, t127209)
}
