//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1194/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1194<F: Float>(t295: F, t6621: F, t305: F, t6635: F, t1275: F, t2376: F, t1004: F, t6660: F, t19444: F, t970: F, t18856: F, t18896: F, t2788: F, t4965: F, t18900: F, t18912: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t23462 = t295 * t6621;
    let t23473 = t305 * t6635;
    let t23495 = t2376 * t1275;
    let t23498 = t1004 * t6660;
    let t23689 = t19444 * t970;
    let t23694 = 12.0 * t18856;
    let t23708 = 480.0 * t18896;
    let t23709 = t2788 * t4965;
    let t23711 = 48.0 * t18900;
    let t23715 = 960.0 * t18912;
    (t23462, t23473, t23495, t23498, t23689, t23694, t23708, t23709, t23711, t23715)
}
