//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 613/861 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk613<F: Float>(t13065: F, t825: F, t13064: F, t2685: F, t2684: F, t3247: F, t900: F, t10867: F, t10924: F, t787: F, t9824: F, t3427: F, t871: F, t1020: F, t3113: F, t10628: F, t2365: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t13066 = t825 * t13065;
    let t13069 = t2685 * t13064;
    let t13070 = t2684 * t13069;
    let t13072 = t900 * t3247;
    let t13073 = t10867 * t13072;
    let t13077 = t787 * t10924;
    let t13078 = t13077 * t9824;
    let t13088 = t3427 * t871;
    let t13089 = t1020 * t3113;
    let t13118 = t2365 * t10628;
    (t13066, t13069, t13070, t13072, t13073, t13077, t13078, t13088, t13089, t13118)
}
