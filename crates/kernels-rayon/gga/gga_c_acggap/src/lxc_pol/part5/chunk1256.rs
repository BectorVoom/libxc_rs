//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1256/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1256(t15905: f64, t5855: f64, t3382: f64, t6086: f64, t1165: f64, t3361: f64, t4267: f64, t4521: f64, t1180: f64, t13889: f64, t1760: f64, t1879: f64, t23094: f64, t23098: f64, t23105: f64, t23109: f64, t23111: f64, t3201: f64, t398: f64, t418: f64, t5814: f64, t5862: f64, t930: f64, t955: f64) -> f64 {
    let t23113 = t15905 * t5855;
    let t23115 = t3382 * t6086;
    let t23127 = t3361 * t1165 * t4267 * t4521;
    let t23133 = -0.25724410870841842184e-2_f64 * t23094 - 0.34299214494455789578e-2_f64 * t23098 + 0.17149607247227894789e-2_f64 * t1180 * t13889 * t1760 + 0.34299214494455789578e-2_f64 * t23105 + 0.17149607247227894789e-2_f64 * t23109 + 0.40015750243531754508e-1_f64 * t23111 + 0.12004725073059526352e-1_f64 * t23113 - 0.25724410870841842184e-2_f64 * t23115 - 0.21437009059034868486e-3_f64 * t1180 * t1165 * t5862 * t955 - 0.12862205435420921092e-2_f64 * t1180 * t1165 * t1879 * t930 - 0.34299214494455789578e-2_f64 * t23127 - 0.34299214494455789578e-2_f64 * t418 * t398 * t3201 * t5814;
    t23133
}
