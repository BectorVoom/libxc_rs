//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 694/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk694<F: Float>(t4952: F, t541: F, t1585: F, t518: F, t101: F, t109: F, t1588: F, t4906: F, t106: F, t1589: F, t4929: F, t4934: F, t4937: F, t4939: F, t4943: F, t4945: F, t4947: F, t4950: F) -> (F, F, F, F, F, F, F, F, F) {
    let t4953 = t4952 * t541;
    let t4957 = F::new(1.0) / t1585 / t518;
    let t4958 = t101 * t4957;
    let t4960 = F::new(1.0) / t1588 / t109;
    let t4961 = t4906 * t4960;
    let t4965 = F::new(1.0) / t1585 / t106;
    let t4966 = t101 * t4965;
    let t4967 = t4906 * t1589;
    let t4978 = -F::new(0.47063e1) * t4929 + F::new(0.31375333333333333334e1) * t4934 - F::new(0.36604555555555555556e1) * t4937 - F::new(0.16068111111111111111e1) * t4939 + F::new(0.28051666666666666666e0) * t4943 - F::new(0.56103333333333333332e0) * t4945 - F::new(0.6545388888888888889e0) * t4947 - F::new(0.46308888888888888888e0) * t4950;
    (t4953, t4957, t4958, t4960, t4961, t4965, t4966, t4967, t4978)
}
