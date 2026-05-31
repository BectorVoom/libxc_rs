//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1390/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1390<F: Float>(t12741: F, t12755: F, t12772: F, t1573: F, t1577: F, t1578: F, t17889: F, t17892: F, t2080: F, t2084: F, t21351: F, t21353: F, t22888: F, t22899: F, t22904: F, t22924: F, t4363: F, t601: F, t6106: F, t6114: F, t7463: F, t7469: F, t7472: F, t7475: F) -> F {
    let t22927 = F::cast_from(0.32164683177870697974e2_f64) * t12772 * t7463 + F::cast_from(0.58482233974552040708e0_f64) * t22888 * t1578 + F::cast_from(0.11696446794910408142e1_f64) * t17889 * t2084 + F::cast_from(0.11696446794910408142e1_f64) * t6106 * t6114 - F::cast_from(0.11696446794910408142e1_f64) * t12755 * t7469 + F::cast_from(0.58482233974552040708e0_f64) * t4363 * t7472 + F::cast_from(0.58482233974552040708e0_f64) * t1577 * t22899 + F::cast_from(0.17315755899375863299e2_f64) * t12741 * t7475 + F::cast_from(1.0_f64) * t22904 * t1573 + F::cast_from(2.0_f64) * t17892 * t2080 - F::cast_from(0.3109e-1_f64) * t22924 * t601 + t21351 - t21353;
    t22927
}
