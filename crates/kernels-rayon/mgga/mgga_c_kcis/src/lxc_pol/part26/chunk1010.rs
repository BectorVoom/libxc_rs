//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1010/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1010(t12741: f64, t12755: f64, t12772: f64, t1573: f64, t1577: f64, t1578: f64, t17889: f64, t17892: f64, t2080: f64, t2084: f64, t21351: f64, t21353: f64, t22888: f64, t22899: f64, t22904: f64, t22924: f64, t4363: f64, t601: f64, t6106: f64, t6114: f64, t7463: f64, t7469: f64, t7472: f64, t7475: f64) -> f64 {
    let t22927 = 0.32164683177870697974e2_f64 * t12772 * t7463 + 0.58482233974552040708e0_f64 * t22888 * t1578 + 0.11696446794910408142e1_f64 * t17889 * t2084 + 0.11696446794910408142e1_f64 * t6106 * t6114 - 0.11696446794910408142e1_f64 * t12755 * t7469 + 0.58482233974552040708e0_f64 * t4363 * t7472 + 0.58482233974552040708e0_f64 * t1577 * t22899 + 0.17315755899375863299e2_f64 * t12741 * t7475 + 1.0_f64 * t22904 * t1573 + 2.0_f64 * t17892 * t2080 - 0.3109e-1_f64 * t22924 * t601 + t21351 - t21353;
    t22927
}
