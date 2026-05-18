//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 583/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk583<F: Float>(t1737: F, t1746: F, t4948: F, t1736: F, t4929: F, t633: F, t1706: F, t1726: F, t1735: F, t1747: F, t45: F, t4850: F, t4853: F, t4858: F, t4860: F, t4904: F, t4909: F, t4912: F, t4920: F, t4924: F, t4931: F, t621: F, t634: F) -> (F, F, F, F, F, F, F) {
    let t4950 = t1737 * t4948 * t1746;
    let t4953 = t1736 * t1736;
    let t4954 = F::new(1.0) / t4953;
    let t4955 = t4954 * t4929;
    let t4956 = t633 * t633;
    let t4957 = F::new(1.0) / t4956;
    let t4958 = t4955 * t4957;
    let t4961 = -F::new(0.62182e-1) * t4850 * t621 + F::new(2.0) * t4853 * t1726 - F::new(2.0) * t4858 * t4860 + F::new(1.0) * t1706 * t4904 + F::new(0.16081824322151104822e2) * t4909 * t4912 + F::new(0.19751789702565206229e-1) * t45 * t4920 * t634 - F::new(0.11696446794910408142e1) * t4924 * t1747 + F::new(0.11696446794910408142e1) * t1735 * t4931 - F::new(0.58482233974552040708e0) * t1735 * t4950 - F::new(0.17315755899375863299e2) * t1735 * t4958;
    (t4950, t4953, t4954, t4956, t4957, t4958, t4961)
}
