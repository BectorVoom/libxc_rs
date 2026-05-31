//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 861/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk861<F: Float>(t10218: F, t13710: F, t13714: F, t13717: F, t13781: F, t18828: F, t18830: F, t18833: F, t18835: F, t18853: F, t9691: F, t10199: F, t10202: F, t1036: F, t13747: F, t13750: F, t1670: F, t18685: F, t18803: F, t18808: F, t18817: F, t18824: F, t245: F, t3078: F, t3081: F, t4625: F, t4647: F, t4654: F, t6320: F, t6338: F, t934: F) -> F {
    let t18854 = F::cast_from(0.14865e-1_f64) * t18828 - F::cast_from(0.1982e-1_f64) * t18830 - F::cast_from(0.991e-2_f64) * t18833 + F::cast_from(0.1982e-1_f64) * t18835 - t10218 - F::cast_from(0.18344444444444444444e-2_f64) * t9691 - F::cast_from(0.36688888888888888888e-2_f64) * t13710 + t13781 - F::cast_from(0.55033333333333333332e-2_f64) * t13714 + F::cast_from(0.55033333333333333332e-2_f64) * t13717 + t18853;
    let t18857 = F::cast_from(3.0_f64) / F::cast_from(16.0_f64) * t10199 * t18803 - t10202 * t6320 / F::cast_from(8.0_f64) - t3078 * t18808 / F::cast_from(4.0_f64) - t13747 * t4647 / F::cast_from(4.0_f64) + t13750 * t1670 / F::cast_from(2.0_f64) + t4654 * t4625 / F::cast_from(2.0_f64) - t3078 * t18817 / F::cast_from(8.0_f64) + t3081 * t6338 / F::cast_from(4.0_f64) + t1036 * t18685 / F::cast_from(4.0_f64) + t18824 * t934 / F::cast_from(4.0_f64) + t245 * t18854 / F::cast_from(2.0_f64);
    t18857
}
