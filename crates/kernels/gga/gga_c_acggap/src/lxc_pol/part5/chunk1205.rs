//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1205/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1205<F: Float>(t1421: F, t301: F, t15393: F, t176: F, t525: F, t1165: F, t3456: F, t4241: F, t5852: F, t13371: F, t13373: F, t16900: F, t16902: F, t16911: F, t16916: F, t16921: F, t16926: F, t16928: F, t16930: F) -> F {
    let t21955 = t1421 * t301;
    let t21958 = t15393 * t176 * t525 * t21955;
    let t21970 = t3456 * t1165 * t5852 * t4241;
    let t21972 = -F::cast_from(0.34299214494455789578e-1_f64) * t16900 + F::cast_from(0.12004725073059526352e-1_f64) * t16902 + F::cast_from(0.17149607247227894789e-1_f64) * t21958 - F::cast_from(0.40015750243531754508e-2_f64) * t13371 - F::cast_from(0.12004725073059526352e-1_f64) * t13373 - F::cast_from(0.85748036236139473944e-3_f64) * t16911 - F::cast_from(0.17149607247227894789e-2_f64) * t16916 - F::cast_from(0.17149607247227894789e-2_f64) * t16921 - F::cast_from(0.85748036236139473944e-3_f64) * t16926 - F::cast_from(0.42874018118069736972e-3_f64) * t16928 - F::cast_from(7.0_f64) / F::cast_from(36.0_f64) * t16930 - F::cast_from(0.25724410870841842183e-2_f64) * t21970;
    t21972
}
