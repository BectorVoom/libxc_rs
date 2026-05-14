//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 876/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk876<F: Float>(t19846: F, t2211: F, t3783: F, t1333: F, t5869: F, t6343: F, t3739: F, t6008: F, t13955: F, t2178: F, t1413: F, t5866: F, t5616: F, t13440: F, t2173: F, t1219: F, t5798: F, sigma0: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t19847 = 0.33163888888888888888e-2 * t19846;
    let t19848 = t2211 * t3783;
    let t19849 = t19848 * sigma0;
    let t19856 = t1333 * t5869;
    let t19857 = 0.33163888888888888888e-2 * t19856;
    let t19861 = t6343 * sigma0;
    let t19926 = t3739 * t6008;
    let t19948 = t13955 * t2178;
    let t19950 = t5866 * t1413;
    let t19951 = t19950 * sigma0;
    let t19966 = t1333 * t5616;
    let t19968 = t2173 * t13440;
    let t19972 = t5798 * t1219;
    (t19847, t19848, t19849, t19856, t19857, t19861, t19926, t19948, t19950, t19951, t19966, t19968, t19972)
}
