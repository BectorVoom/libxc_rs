//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1410/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1410<F: Float>(t132: F, t10325: F, t11204: F, t11209: F, t1793: F, t2002: F, t2028: F, t21911: F, t25930: F, t2688: F, t2750: F, t29765: F, t3627: F, t3925: F, t3938: F, t457: F, t5891: F, t675: F, t7292: F, t9354: F, zeta_threshold: F) -> F {
    let t133 = t132 <= zeta_threshold;
    let t30501 = piecewise3::<f64>(t133, F::new(0.0), F::new(40.0) / F::new(81.0) * t21911 * t3925 * t2028 + F::new(64.0) / F::new(27.0) * t9354 * t29765 - F::new(8.0) / F::new(27.0) * t11204 * t2002 + F::new(32.0) / F::new(9.0) * t2688 * t457 * t2750 - F::new(16.0) / F::new(9.0) * t3627 * t1793 + F::new(16.0) / F::new(3.0) * t3627 * t5891 - F::new(8.0) / F::new(27.0) * t7292 * t3938 * t2028 + F::new(8.0) / F::new(9.0) * t2688 * t10325 * t675 + F::new(4.0) / F::new(9.0) * t11209 * t2002 - t25930);
    t30501
}
