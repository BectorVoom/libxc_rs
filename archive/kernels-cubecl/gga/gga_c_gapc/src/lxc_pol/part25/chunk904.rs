//! GGA_C_GAPC lxc pol — lxc_pol part 25 (v4rho2sigma2_4) CSE chunk 904/1444 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part25_v4rho2sigma2_4_chunk904<F: Float>(t6188: F, t772: F, t3243: F, t10247: F, t10250: F, t10253: F, t10258: F, t10262: F, t10267: F, t10271: F, t10274: F, t10276: F, t10278: F) -> (F, F) {
    let t10280 = t772 * t6188;
    let t10281 = t3243 * t10280;
    let t10283 = F::cast_from(0.69596735221749395468e-7_f64) * t10247 - F::cast_from(0.2087902056652481864e-5_f64) * t10250 - F::cast_from(0.11742981196020707897e-5_f64) * t10253 - F::cast_from(0.74922666485027954031e-6_f64) * t10258 - F::cast_from(0.12374299522427042515e-6_f64) * t10262 + F::cast_from(0.2087902056652481864e-5_f64) * t10267 - F::cast_from(0.11742981196020707897e-4_f64) * t10271 - F::cast_from(0.33406432906439709826e-4_f64) * t10274 + F::cast_from(0.74372214241464483348e-4_f64) * t10276 + F::cast_from(0.23404413911513494211e-4_f64) * t10278 - F::cast_from(0.11742981196020707897e-5_f64) * t10281;
    (t10281, t10283)
}
