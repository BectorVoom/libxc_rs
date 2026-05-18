//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1158/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1158<F: Float>(t5966: F, t997: F, t12855: F, t12862: F, t15851: F, t15853: F, t15855: F, t15871: F, t15891: F, t15902: F, t20875: F, t20882: F, t20888: F, t20890: F) -> F {
    let t20892 = t997 * t5966;
    let t20894 = F::new(0.15117061203111996147e0) * t15851 + F::new(0.30234122406223992295e0) * t15853 - F::new(0.17149607247227894789e-2) * t20875 + F::new(0.34299214494455789578e-2) * t15855 - F::new(0.34299214494455789578e-2) * t15871 - F::new(0.40015750243531754508e-2) * t12855 - F::new(0.85748036236139473944e-3) * t12862 - F::new(0.25724410870841842183e-2) * t15891 + F::new(0.32012600194825403606e-1) * t20882 + F::new(0.7558530601555998074e-1) * t15902 + F::new(0.85748036236139473944e-2) * t20888 + F::new(0.40015750243531754508e-1) * t20890 + F::new(0.32012600194825403606e-1) * t20892;
    t20894
}
