//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 589/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk589<F: Float>(t147: F, t184: F, t4888: F, t21: F, t1078: F, t1079: F, t920: F, t1064: F, t1080: F, t185: F, t3601: F, t4431: F, t4845: F, t5: F, t623: F) -> (F, F, F, F, F, F, F) {
    let t148 = F::cast_from(10000000.0_f64) <= t147;
    let t4889 = t4888 * t184;
    let t4890 = t4889 * t21;
    let t4893 = t1078 * t1078;
    let t4894 = t4893 * t184;
    let t4895 = t4894 * t21;
    let t4898 = t1079 * t920;
    let t4905 = piecewise3::<F>(t148, F::cast_from(0.0_f64), t5 * t4845 * t21 / F::cast_from(4.0_f64) + t3601 * t1080 / F::cast_from(2.0_f64) + t5 * t1064 * t920 / F::cast_from(2.0_f64) + t623 * t4890 / F::cast_from(4.0_f64) + t623 * t4895 / F::cast_from(4.0_f64) + t623 * t4898 / F::cast_from(2.0_f64) + t5 * t185 * t4431 / F::cast_from(4.0_f64));
    (t4889, t4890, t4893, t4894, t4895, t4898, t4905)
}
