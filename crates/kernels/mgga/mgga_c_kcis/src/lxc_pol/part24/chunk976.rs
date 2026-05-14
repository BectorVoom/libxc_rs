//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 976/1171 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk976<F: Float>(t28093: F, t7772: F, t1268: F, t1851: F, t922: F, t3515: F, t5281: F, t5310: F, t1262: F, t1646: F, t26961: F, t330: F, t3622: F, t1267: F, t1071: F, t4547: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t28094 = t7772 * t28093;
    let t28096 = t1268 * t1851;
    let t28097 = t28096 * t922;
    let t28098 = t3515 * t28097;
    let t28101 = t5281 * t922;
    let t28102 = t5310 * t28101;
    let t28105 = t1646 * t1262;
    let t28106 = t26961 * t28105;
    let t28107 = t3515 * t28106;
    let t28110 = t3622 * t330;
    let t28111 = t1646 * t1267;
    let t28112 = t28110 * t28111;
    let t28113 = t5310 * t28112;
    let t28116 = t1268 * t1071;
    let t28117 = t28116 * t4547;
    (t28094, t28097, t28098, t28101, t28102, t28105, t28106, t28107, t28110, t28111, t28112, t28113, t28116, t28117)
}
