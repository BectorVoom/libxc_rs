//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3137/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3137<F: Float>(t12248: F, t1732: F, t12415: F, t12222: F, t5192: F, t1196: F, t45289: F, t5205: F, t12235: F, t16673: F, t3531: F, t12361: F, t16655: F) -> (F, F, F, F, F, F) {
    let t57818 = t12248 * t1732;
    let t57820 = F::cast_from(0.2894756309764656312e3_f64) * t57818 * t12415;
    let t57822 = F::cast_from(0.51947577317044391277e2_f64) * t5192 * t12222;
    let t57825 = F::cast_from(0.17315859105681463759e2_f64) * t1196 * t5205 * t45289;
    let t57827 = F::cast_from(0.35089341735807877242e1_f64) * t5192 * t12235;
    let t57829 = F::cast_from(0.10389515463408878255e3_f64) * t3531 * t16673;
    let t57831 = F::cast_from(6.0_f64) * t12361 * t16655;
    (t57820, t57822, t57825, t57827, t57829, t57831)
}
