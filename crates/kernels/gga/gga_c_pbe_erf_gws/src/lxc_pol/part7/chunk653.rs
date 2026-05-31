//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 653/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk653<F: Float>(t5152: F, t639: F, t1692: F, t617: F, t2677: F, t1620: F, t1726: F, t633: F, t4359: F, t220: F, t186: F, t616: F) -> (F, F, F, F, F, F, F, F, F) {
    let t5154 = F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t639 * t5152;
    let t5155 = t1692 * t617;
    let t5156 = t2677 * t5155;
    let t5158 = F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t1620 * t5156;
    let t5160 = F::cast_from(2.0_f64) / F::cast_from(5.0_f64) * t633 * t1726;
    let t5162 = -F::cast_from(3.0_f64) * t4359;
    let t5163 = t220 * t5162;
    let t5164 = t186 * t5163;
    let t5166 = F::cast_from(4.0_f64) / F::cast_from(15.0_f64) * t616 * t5164;
    (t5154, t5155, t5156, t5158, t5160, t5162, t5163, t5164, t5166)
}
