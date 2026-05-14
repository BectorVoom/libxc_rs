//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 806/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk806<F: Float>(t16499: F, t16521: F, t16549: F, t16566: F, t106: F, t1299: F, t13300: F, t16443: F, t16456: F, t16460: F, t167: F, t3454: F, t3461: F, t4675: F, t4723: F, t670: F, t6977: F, t9794: F) -> (F, F) {
    let t16568 = t16499 + t16521 + t16549 + t16566;
    let t16572 = 0.27818116767324025134e1 * t106 * t16443 * t167 - 0.83454350301972075402e1 * t106 * t13300 * t1299 + 0.16690870060394415081e2 * t106 * t9794 * t4675 - 0.83454350301972075402e1 * t106 * t3454 * t4723 - 0.1669087006039441508e2 * t106 * t6977 * t16456 + 0.16690870060394415081e2 * t3461 * t16460 - 0.27818116767324025134e1 * t106 * t670 * t16568;
    (t16568, t16572)
}
