//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1256/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1256<F: Float>(t15905: F, t5855: F, t3382: F, t6086: F, t1165: F, t3361: F, t4267: F, t4521: F, t1180: F, t13889: F, t1760: F, t1879: F, t23094: F, t23098: F, t23105: F, t23109: F, t23111: F, t3201: F, t398: F, t418: F, t5814: F, t5862: F, t930: F, t955: F) -> F {
    let t23113 = t15905 * t5855;
    let t23115 = t3382 * t6086;
    let t23127 = t3361 * t1165 * t4267 * t4521;
    let t23133 = -F::new(0.25724410870841842184e-2) * t23094 - F::new(0.34299214494455789578e-2) * t23098 + F::new(0.17149607247227894789e-2) * t1180 * t13889 * t1760 + F::new(0.34299214494455789578e-2) * t23105 + F::new(0.17149607247227894789e-2) * t23109 + F::new(0.40015750243531754508e-1) * t23111 + F::new(0.12004725073059526352e-1) * t23113 - F::new(0.25724410870841842184e-2) * t23115 - F::new(0.21437009059034868486e-3) * t1180 * t1165 * t5862 * t955 - F::new(0.12862205435420921092e-2) * t1180 * t1165 * t1879 * t930 - F::new(0.34299214494455789578e-2) * t23127 - F::new(0.34299214494455789578e-2) * t418 * t398 * t3201 * t5814;
    t23133
}
