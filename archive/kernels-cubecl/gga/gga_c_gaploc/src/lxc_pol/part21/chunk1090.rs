//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1090/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1090<F: Float>(t1699: F, t3225: F, t7102: F, t871: F, t2628: F, t7340: F, t22537: F, t822: F, t20671: F, t22634: F, t2012: F, t9804: F) -> (F, F, F, F, F, F) {
    let t27860 = t3225 * t1699;
    let t27868 = t7102 * t871;
    let t28022 = F::cast_from(0.11916829983950142223e0_f64) * t7340 * t2628;
    let t28069 = t822 * t22537;
    let t28072 = F::cast_from(0.85206502119823888169e0_f64) * t28069 * t20671 * t22634;
    let t28073 = t2012 * t9804;
    (t27860, t27868, t28022, t28069, t28072, t28073)
}
