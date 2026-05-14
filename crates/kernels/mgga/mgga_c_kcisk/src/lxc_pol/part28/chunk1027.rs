//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1027/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1027<F: Float>(t1725: F, t23705: F, t45: F, t8740: F, t10983: F, t1706: F, t1735: F, t1747: F, t17567: F, t17656: F, t23516: F, t23519: F, t23522: F, t23525: F, t23529: F, t23532: F, t23634: F, t23654: F, t2432: F, t4853: F, t4858: F, t4909: F, t4924: F, t621: F, t7139: F, t7151: F, t7158: F, t7177: F, t8730: F, t8748: F, t8769: F) -> (F,) {
    let t23706 = t23705 * t1725;
    let t23709 = t45 * t8740;
    let t23714 = 0.32163648644302209644e2 * t17567 * t7139 + 6.0 * t4909 * t23516 - 4.0 * t4858 * t23519 - 0.96490945932906628932e2 * t10983 * t23522 - 2.0 * t4858 * t23525 + 0.16081824322151104822e2 * t4909 * t23529 + 0.32163648644302209644e2 * t4909 * t23532 - 0.11696446794910408142e1 * t7151 * t7177 + 0.11696446794910408142e1 * t4924 * t8748 - 0.58482233974552040708e0 * t1735 * t23634 - 0.17315755899375863299e2 * t4924 * t8769 - 0.62182e-1 * t23654 * t621 - 0.11696446794910408142e1 * t17656 * t2432 + 1.0 * t4853 * t8730 + 1.0 * t1706 * t23706 - 0.58482233974552040708e0 * t23709 * t1747 + 0.23392893589820816284e1 * t7151 * t7158;
    (t23714,)
}
