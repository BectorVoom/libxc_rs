//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 668/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk668<F: Float>(t1725: F, t582: F, t211: F, t1775: F, t612: F, t1680: F, t1872: F, t2660: F, t1879: F, t5162: F, t198: F, t186: F) -> (F, F, F, F, F, F, F, F, F) {
    let t5322 = t582 * t1725;
    let t5323 = t211 * t5322;
    let t5324 = F::cast_from(4.0_f64) / F::cast_from(15.0_f64) * t5323;
    let t5326 = F::cast_from(2.0_f64) / F::cast_from(5.0_f64) * t1775 * t612;
    let t5328 = F::cast_from(4.0_f64) / F::cast_from(5.0_f64) * t1680 * t612;
    let t5330 = F::cast_from(4.0_f64) / F::cast_from(5.0_f64) * t2660 * t1872;
    let t5332 = F::cast_from(4.0_f64) / F::cast_from(5.0_f64) * t1879 * t1872;
    let t5333 = -t5162;
    let t5334 = t198 * t5333;
    let t5335 = t186 * t5334;
    (t5322, t5324, t5326, t5328, t5330, t5332, t5333, t5334, t5335)
}
