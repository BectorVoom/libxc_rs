//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 862/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk862<F: Float>(t226: F, t7: F, t7236: F, t7271: F, t1735: F, t7632: F, t1750: F, t1795: F, t1775: F, t1868: F, t1680: F, t1872: F, t7839: F) -> (F, F, F, F, F, F) {
    let pi = F::cast_from(M_PI);
    let t16595 = F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t226 * (-F::cast_from(0.42777777777777777777e1_f64) * t7271 + F::cast_from(220.0_f64) / F::cast_from(81.0_f64) * t7236) * pi * t7;
    let t16597 = F::cast_from(16.0_f64) / F::cast_from(5.0_f64) * t7632 * t1735;
    let t16599 = F::cast_from(8.0_f64) / F::cast_from(5.0_f64) * t1750 * t1795;
    let t16601 = F::cast_from(4.0_f64) / F::cast_from(5.0_f64) * t1775 * t1868;
    let t16603 = F::cast_from(8.0_f64) / F::cast_from(5.0_f64) * t1680 * t1868;
    let t16605 = F::cast_from(16.0_f64) / F::cast_from(5.0_f64) * t7839 * t1872;
    (t16595, t16597, t16599, t16601, t16603, t16605)
}
