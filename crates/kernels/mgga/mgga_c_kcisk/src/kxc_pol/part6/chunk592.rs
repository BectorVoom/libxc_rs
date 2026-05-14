//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 592/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk592<F: Float>(t1737: F, t1746: F, t8763: F, t4954: F, t8746: F, t4957: F, t1706: F, t1735: F, t2418: F, t2432: F, t45: F, t4858: F, t4909: F, t621: F, t634: F, t7091: F, t7151: F, t8692: F, t8698: F, t8730: F, t8733: F, t8740: F, t8748: F) -> (F, F, F) {
    let t8765 = t1737 * t8763 * t1746;
    let t8768 = t4954 * t8746;
    let t8769 = t8768 * t4957;
    let t8772 = -0.62182e-1 * t8692 * t621 + 2.0 * t7091 * t2418 - 2.0 * t4858 * t8698 + 1.0 * t1706 * t8730 + 0.16081824322151104822e2 * t4909 * t8733 + 0.19751789702565206229e-1 * t45 * t8740 * t634 - 0.11696446794910408142e1 * t7151 * t2432 + 0.11696446794910408142e1 * t1735 * t8748 - 0.58482233974552040708e0 * t1735 * t8765 - 0.17315755899375863299e2 * t1735 * t8769;
    (t8765, t8769, t8772)
}
