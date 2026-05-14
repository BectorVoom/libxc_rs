//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 881/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk881<F: Float>(t15870: F, t6716: F, t2063: F, t4797: F, t5185: F, t5184: F, t5182: F, t10414: F, t6982: F, t2571: F, t4648: F, t5192: F, t6987: F, t6663: F, t6669: F, t1336: F, t140: F, t705: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t15871 = t15870 * t6716;
    let t15874 = t5185 * t2063 * t4797;
    let t15875 = t5184 * t15874;
    let t15876 = t5182 * t15875;
    let t15878 = t10414 * t6982;
    let t15880 = t2571 * t4648;
    let t15881 = t5192 * t15880;
    let t15882 = t5182 * t15881;
    let t15884 = t10414 * t6987;
    let t15886 = t10414 * t6663;
    let t15888 = t10414 * t6669;
    let t15891 = t140 * t1336 * t705;
    (t15871, t15874, t15876, t15878, t15880, t15882, t15884, t15886, t15888, t15891)
}
