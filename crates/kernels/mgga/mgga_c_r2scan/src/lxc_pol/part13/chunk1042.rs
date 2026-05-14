//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 1042/1115 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk1042<F: Float>(t3588: F, t37932: F, t10894: F, t8243: F, t10810: F, t2184: F, t7629: F, t7625: F, t26314: F, t37755: F, t39841: F, t39958: F, t39963: F, t39965: F, t39968: F, t39969: F, t39972: F, t39975: F) -> (F,) {
    let t39977 = t37932 * t3588;
    let t39979 = t10894 * t8243;
    let t39980 = 0.10975748638225852664e-1 * t39979;
    let t39982 = t2184 * t10810 * t7629;
    let t39983 = 0.46230515946956099004e0 * t39982;
    let t39984 = t10894 * t7625;
    let t39985 = 0.54878743191129263322e-2 * t39984;
    let t39987 = t37755 * t39841 * t26314;
    let t39989 = 0.93149212406257582491e-1 * t39958 + t39963 + t39965 + t39968 + 0.14282990759302185291e-1 * t39969 + 0.2600466522016280569e0 * t39972 + 0.10401866088065122276e1 * t39975 - 0.21341733463216935736e0 * t39977 - t39980 - t39983 + t39985 + 0.13099107994629972538e-1 * t39987;
    (t39989,)
}
