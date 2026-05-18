//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1182/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1182<F: Float>(t5529: F, t997: F, t12854: F, t1817: F, t1165: F, t1173: F, t13121: F, t13128: F, t13133: F, t13135: F, t13137: F, t13146: F, t16612: F, t16625: F, t1889: F, t407: F, t5688: F, t930: F) -> F {
    let t21484 = t997 * t5529;
    let t21486 = t12854 * t1817;
    let t21489 = -F::new(0.40015750243531754508e-2) * t16612 + F::new(0.17149607247227894789e-2) * t1173 * t1165 * t5688 * t407 + F::new(0.85748036236139473944e-3) * t1173 * t1165 * t1889 * t930 - F::new(0.17149607247227894789e-2) * t16625 - F::new(0.34299214494455789578e-2) * t13121 - F::new(0.10289764348336736874e-1) * t13128 - F::new(0.17149607247227894789e-2) * t13133 + F::new(0.51448821741683684366e-2) * t13135 - F::new(0.85748036236139473945e-2) * t13137 - F::new(0.24009450146119052706e-1) * t21484 - F::new(0.40015750243531754508e-2) * t21486 - F::new(0.25724410870841842183e-2) * t13146;
    t21489
}
