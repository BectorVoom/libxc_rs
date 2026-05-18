//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1010/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1010<F: Float>(t168: F, t5589: F, t738: F, t1365: F, t1452: F, t153: F, t18046: F, t274: F, t1457: F, t700: F, t1383: F, t762: F) -> (F, F, F, F, F) {
    let t18352 = t168 * t5589 * t738;
    let t18355 = t153 * t1365 * t1452;
    let t18359 = F::new(0.19192636997366703204e2) * t153 * t18046 * t274;
    let t18360 = t1457 * t700;
    let t18363 = F::new(0.10051538464260528225e1) * t762 * t1383;
    (t18352, t18355, t18359, t18360, t18363)
}
