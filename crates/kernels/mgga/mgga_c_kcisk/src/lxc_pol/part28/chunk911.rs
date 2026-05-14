//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 911/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk911<F: Float>(t1814: F, t2372: F, t11313: F, t2514: F, t3521: F, t7031: F, t7036: F, t6771: F, t682: F, t12760: F, t139: F, t41: F, t7017: F, t1417: F, t7061: F, t7040: F) -> (F, F, F, F, F, F, F, F) {
    let t16892 = t1814 * t2372;
    let t16897 = t11313 * t2514;
    let t16900 = 0.98556445e-3 * t3521 * t7031;
    let t16902 = 0.19711289e-2 * t3521 * t7036;
    let t16917 = t682 * t6771;
    let t16940 = t139 * t12760 * t41;
    let t16941 = t16940 * t7017;
    let t16945 = 0.13140859333333333333e-2 * t1417 * t7061;
    let t16957 = 0.19711289e-2 * t1417 * t7040;
    (t16892, t16897, t16900, t16902, t16917, t16941, t16945, t16957)
}
