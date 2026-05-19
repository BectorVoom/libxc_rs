//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 518/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk518<F: Float>(t221: F, t3479: F, t1755: F, t2760: F, t3423: F, t3427: F, t3431: F, t173: F, t184: F) -> (F, F, F, F) {
    let t3481 = F::new(2.0) / F::new(15.0) * t3479 * t221;
    let t3486 = -t1755 - F::cast_from(0.12594444444444444445e-2_f64) * t2760 + F::cast_from(0.12594444444444444445e-2_f64) * t3423 - F::cast_from(0.37783333333333333334e-2_f64) * t3427 + F::cast_from(0.18891666666666666667e-2_f64) * t3431;
    let t3487 = t173 * t3486;
    let t3488 = t3487 * t184;
    (t3481, t3486, t3487, t3488)
}
