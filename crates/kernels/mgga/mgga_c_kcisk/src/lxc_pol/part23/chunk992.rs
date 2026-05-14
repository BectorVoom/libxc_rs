//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 992/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk992<F: Float>(t1319: F, t20052: F, t1322: F, t5601: F, t3970: F, t6171: F, t3966: F, t2159: F, t3969: F, t3974: F, t6157: F, t3988: F, t6205: F, t6204: F, t1309: F, t1315: F, t13795: F, t13821: F, t13839: F, t2164: F, t3935: F, t3944: F, t6207: F) -> (F, F, F) {
    let t20067 = t20052 * t1319;
    let t20068 = t5601 * t1322;
    let t20069 = t20067 * t20068;
    let t20072 = t3970 * t6171;
    let t20075 = 0.11993859144118211475e-1 * t3966 * t6171;
    let t20084 = t2159 * t3969;
    let t20088 = 0.11993859144118211475e-1 * t6157 * t3974;
    let t20091 = t6205 * t3988;
    let t20092 = t6204 * t20091;
    let t20095 = -0.71963154864709268852e-1 * t3935 * t20069 - 0.31983624384315230601e-1 * t20072 + t20075 + 0.1759099341137337683e0 * t13821 * t2164 - 0.95950873152945691802e-1 * t13839 * t2164 - 0.35981577432354634426e-1 * t6157 * t3944 + 0.17990788716177317213e-1 * t13795 * t2164 - 0.95950873152945691804e-1 * t20084 * t1315 + t20088 + 0.21588946459412780656e0 * t3966 * t6207 + 0.10794473229706390328e0 * t1309 * t20092;
    (t20067, t20084, t20095)
}
