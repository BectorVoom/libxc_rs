//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 879/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk879<F: Float>(t16780: F, t5175: F, t590: F, t418: F, t5177: F, t572: F, t587: F, t1820: F, t1866: F, t562: F, t610: F, t7703: F) -> (F, F, F) {
    let t16781 = F::cast_from(16.0_f64) / F::cast_from(15.0_f64) * t16780;
    let t16782 = t590 * t5175;
    let t16787 = F::cast_from(32.0_f64) / F::cast_from(15.0_f64) * t587 * t16782 * t5177 * t572 * t418;
    let t16792 = F::cast_from(32.0_f64) / F::cast_from(5.0_f64) * t1820 * t7703 * t610 * t1866 * t562;
    (t16781, t16787, t16792)
}
