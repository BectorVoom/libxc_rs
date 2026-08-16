//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1094/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1094<F: Float>(t4888: F, t4893: F, t1064: F, t1078: F, t16612: F, t184: F, t185: F, t20044: F, t20990: F, t21: F, t21086: F, t21092: F, t3601: F, t3664: F, t4431: F, t4845: F, t4889: F, t4894: F, t4898: F, t5: F, t623: F, t85501: F, t87219: F, t87814: F, t920: F) -> F {
    let t87827 = t4888 * t4888;
    let t87835 = t4893 * t4893;
    let t87840 = F::cast_from(3.0_f64) * t3601 * t21092 + t623 * t21086 * t1078 * t3664 + F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t623 * t4894 * t21 * t4888 + F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t5 * t4845 * t4431 + t5 * t20990 * t920 + t5 * t185 * t85501 / F::cast_from(4.0_f64) + t5 * (t87219 + t87814) * t184 * t21 / F::cast_from(4.0_f64) + t5 * t1064 * t20044 + F::cast_from(3.0_f64) * t16612 * t4898 + F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t623 * t4889 * t4431 + F::cast_from(3.0_f64) / F::cast_from(4.0_f64) * t623 * t87827 * t184 * t21 + F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t623 * t4894 * t4431 + t623 * t87835 * t184 * t21 / F::cast_from(4.0_f64);
    t87840
}
