//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 997/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk997<F: Float>(t338: F, t25: F, t6212: F, t1309: F, t3970: F, t6196: F, t164: F, t2169: F, t19710: F, t1320: F, t1310: F, t1324: F, t13821: F, t13851: F, t13859: F, t20097: F, t20169: F, t2170: F, t3963: F, t3966: F, t6157: F, t6213: F) -> (F, F) {
    let t400 = 0.0 < t338;
    let t20175 = t25 * t6212;
    let t20177 = 0.35981577432354634426e-1 * t1309 * t20175;
    let t20182 = t3970 * t6196;
    let t20184 = t164 * t2169;
    let t20185 = t1309 * t20184;
    let t20188 = piecewise3(t400, t19710, -t19710);
    let t20189 = t1320 * t20188;
    let t20190 = t1310 * t20189;
    let t20193 = 0.17990788716177317213e-1 * t13851 + 0.1759099341137337683e0 * t13859 - 0.11993859144118211475e-1 * t20169 - 0.10794473229706390328e0 * t3966 * t6213 + 0.10794473229706390328e0 * t6157 * t3963 - t20177 - 0.10794473229706390328e0 * t20097 * t1324 - 0.52772980234120130494e0 * t13821 * t2170 + 0.95950873152945691804e-1 * t20182 + 0.11993859144118211475e-1 * t20185 - 0.5397236614853195164e-1 * t1309 * t20190;
    (t20188, t20193)
}
