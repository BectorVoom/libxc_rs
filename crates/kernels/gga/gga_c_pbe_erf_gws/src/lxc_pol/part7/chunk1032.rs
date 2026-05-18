//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1032/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1032<F: Float>(t1321: F, t18638: F, t18639: F, t470: F, t1396: F, t4793: F, t1289: F, t1292: F, t13: F, t18515: F, t1276: F, t1285: F, t1291: F) -> (F, F, F, F, F) {
    let t18641 = t1321 * t1321;
    let t18642 = F::new(1.0) / t18641;
    let t18645 = F::new(0.91080982599109921211e5) * t470 * t18638 * t18639 * t18642;
    let t18646 = t4793 * t1396;
    let t18647 = F::new(0.35089340384731224426e1) * t18646;
    let t18648 = t1289 * t1289;
    let t18651 = t1292 * t1292;
    let t18655 = F::new(0.24954977986735470917e5) * t13 / t18648 * t18515 / t18651;
    let t18658 = F::new(36.0) * t1291 * t1276 * t1285;
    (t18642, t18645, t18647, t18655, t18658)
}
