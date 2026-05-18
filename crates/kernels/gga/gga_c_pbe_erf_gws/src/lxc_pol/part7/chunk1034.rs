//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1034/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1034<F: Float>(t43: F, t1285: F, t1274: F, t404: F, t1399: F, t4788: F, t260: F, t1402: F, t1403: F, t1407: F, t16669: F, t16679: F, t16746: F, t4360: F, t47: F, t4757: F, t4760: F, zeta_threshold: F) -> (F, F, F, F) {
    let t44 = t43 <= zeta_threshold;
    let t18664 = t1285 * t1285;
    let t18667 = F::new(6.0) * t1274 * t18664 * t404;
    let t18668 = t1399 * t4788;
    let t18669 = F::new(0.23392893589820816284e1) * t18668;
    let t18670 = F::new(1.0) / t260;
    let t18683 = piecewise3::<f64>(t44, F::new(0.0), F::new(40.0) / F::new(81.0) * t18670 * t16669 - F::new(16.0) / F::new(9.0) * t4757 * t1403 * t1407 + F::new(4.0) / F::new(3.0) * t1402 * t16679 + F::new(16.0) / F::new(9.0) * t4760 * t4360 + F::new(4.0) / F::new(3.0) * t47 * t16746);
    (t18664, t18667, t18669, t18683)
}
