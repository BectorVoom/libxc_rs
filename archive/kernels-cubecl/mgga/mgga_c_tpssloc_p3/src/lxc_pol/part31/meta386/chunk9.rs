//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1374/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1374<F: Float>(t17748: F, t4531: F, t4540: F, t7577: F, t4546: F, t343: F, t5842: F, t984: F, t2970: F, t5824: F, t973: F, t10226: F, t13782: F, t13787: F, t13790: F, t13825: F, t17742: F, t17745: F, t2960: F, t2986: F, t5825: F) -> F {
    let t17749 = t4531 * t17748;
    let t17752 = t7577 * t4540;
    let t17753 = t4546 * t17752;
    let t17757 = t5842 * t984 * t343;
    let t17758 = t4546 * t17757;
    let t17763 = t2970 * t5824;
    let t17764 = t973 * t17763;
    let t17766 = -t13782 + t13787 - t13790 - F::cast_from(0.6172839506172839506e-4_f64) * t10226 + F::cast_from(0.11111111111111111111e-2_f64) * t2986 * t17742 - F::cast_from(0.74074074074074074072e-3_f64) * t2986 * t17745 - F::cast_from(0.55555555555555555554e-3_f64) * t2986 * t17749 - F::cast_from(0.16666666666666666666e-2_f64) * t973 * t17753 - F::cast_from(0.83333333333333333332e-3_f64) * t973 * t17758 + F::cast_from(0.14814814814814814814e-2_f64) * t2960 * t5825 - F::cast_from(0.18518518518518518518e-3_f64) * t17764 - t13825;
    t17766
}
