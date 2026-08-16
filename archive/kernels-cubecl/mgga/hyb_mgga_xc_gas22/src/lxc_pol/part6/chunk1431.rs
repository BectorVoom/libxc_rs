//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1431/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1431<F: Float>(t11552: F, t2824: F, t11556: F, t5204: F, t9520: F, t11540: F, t22746: F, t30908: F, t30919: F, t30993: F, t30996: F, t7806: F, t7811: F, t9521: F, t9527: F, t9535: F, t9549: F, t9558: F) -> F {
    let t31004 = t11552 * t2824;
    let t31009 = t11556 * t2824;
    let t31015 = t5204 * t9520;
    let t31030 = -F::cast_from(1600.0_f64) / F::cast_from(3.0_f64) * t9549 * t30919 + F::cast_from(32.0_f64) * t7806 * t31004 + F::cast_from(1600.0_f64) / F::cast_from(3.0_f64) * t9549 * t30993 - F::cast_from(112.0_f64) / F::cast_from(3.0_f64) * t9558 * t31009 + F::cast_from(160.0_f64) * t22746 * t11540 * t2824 + F::cast_from(400.0_f64) / F::cast_from(9.0_f64) * t31015 * t9535 - F::cast_from(352.0_f64) / F::cast_from(3.0_f64) * t7806 * t30908 + F::cast_from(32.0_f64) / F::cast_from(9.0_f64) * t7811 * t30996 - F::cast_from(1600.0_f64) / F::cast_from(27.0_f64) * t9521 * t30919 + F::cast_from(32.0_f64) / F::cast_from(9.0_f64) * t7811 * t31004 + F::cast_from(1600.0_f64) / F::cast_from(27.0_f64) * t9521 * t30993 - F::cast_from(16.0_f64) / F::cast_from(3.0_f64) * t9527 * t31009;
    t31030
}
