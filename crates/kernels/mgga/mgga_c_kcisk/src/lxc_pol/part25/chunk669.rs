//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 669/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk669<F: Float>(t1737: F, t1746: F, t7175: F, t2430: F, t4954: F, t1744: F, t4957: F, t1706: F, t1726: F, t1735: F, t1747: F, t2418: F, t2432: F, t45: F, t4853: F, t4858: F, t4909: F, t4924: F, t621: F, t634: F, t7088: F, t7091: F, t7096: F, t7135: F, t7139: F, t7147: F, t7151: F, t7158: F) -> (F, F, F, F, F) {
    let t7177 = t1737 * t7175 * t1746;
    let t7180 = t4954 * t2430;
    let t7181 = t4957 * t1744;
    let t7182 = t7180 * t7181;
    let t7185 = -0.62182e-1 * t7088 * t621 + 1.0 * t7091 * t1726 + 1.0 * t4853 * t2418 - 2.0 * t4858 * t7096 + 1.0 * t1706 * t7135 + 0.16081824322151104822e2 * t4909 * t7139 + 0.19751789702565206229e-1 * t45 * t7147 * t634 - 0.58482233974552040708e0 * t7151 * t1747 - 0.58482233974552040708e0 * t4924 * t2432 + 0.11696446794910408142e1 * t1735 * t7158 - 0.58482233974552040708e0 * t1735 * t7177 - 0.17315755899375863299e2 * t1735 * t7182;
    (t7177, t7180, t7181, t7182, t7185)
}
