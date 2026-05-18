//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 639/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk639<F: Float>(t255: F, t28298: F, t10051: F, t1449: F, t3864: F, t6917: F, t9787: F, t1091: F, t24599: F, t2606: F, t24793: F, t3870: F) -> (F, F, F, F, F, F, F, F) {
    let t28299 = t28298 * t255;
    let t28300 = t10051 * t1449;
    let t28301 = t28300 * t3864;
    let t28302 = t28299 * t28301;
    let t28305 = t9787 * t6917;
    let t28308 = t24599 * t1091;
    let t28309 = t2606 * t28308;
    let t28312 = t24793 * t3870;
    (t28299, t28300, t28301, t28302, t28305, t28308, t28309, t28312)
}
