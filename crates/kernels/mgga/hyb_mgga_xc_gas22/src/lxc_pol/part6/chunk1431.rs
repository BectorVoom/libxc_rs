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
    let t31030 = -F::new(1600.0) / F::new(3.0) * t9549 * t30919 + F::new(32.0) * t7806 * t31004 + F::new(1600.0) / F::new(3.0) * t9549 * t30993 - F::new(112.0) / F::new(3.0) * t9558 * t31009 + F::new(160.0) * t22746 * t11540 * t2824 + F::new(400.0) / F::new(9.0) * t31015 * t9535 - F::new(352.0) / F::new(3.0) * t7806 * t30908 + F::new(32.0) / F::new(9.0) * t7811 * t30996 - F::new(1600.0) / F::new(27.0) * t9521 * t30919 + F::new(32.0) / F::new(9.0) * t7811 * t31004 + F::new(1600.0) / F::new(27.0) * t9521 * t30993 - F::new(16.0) / F::new(3.0) * t9527 * t31009;
    t31030
}
