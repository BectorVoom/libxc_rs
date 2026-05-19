//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 858/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk858<F: Float>(t1317: F, t4756: F, t201: F, t3318: F, t104: F, t16310: F, t16315: F, t16318: F, t16319: F, t16572: F, t3316: F, t3539: F, t6359: F, t6437: F, t6449: F, t6709: F, t6711: F, t6766: F, t714: F, t95: F) -> (F, F) {
    let t16577 = t4756 * t1317;
    let t16578 = t16577 * t201;
    let t16579 = t3318 * t16578;
    let t16582 = -t6709 + t6359 + F::cast_from(0.51689762869806860992e-2_f64) * t95 * t104 * t16310 * t6766 + F::cast_from(0.46520786582826174894e-1_f64) * t3539 * t16315 + t6711 + t16318 - t16319 + F::cast_from(0.25844881434903430496e-2_f64) * t95 * t104 * t16572 * t714 + F::new(3.0) / F::new(2.0) * t3316 * t16579 - t6437 + t6449;
    (t16579, t16582)
}
