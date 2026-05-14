//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1428/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1428<F: Float>(t35382: F, t4998: F, t9740: F, t122596: F, t2804: F, t34444: F, t34484: F, t116723: F, t116731: F, t118091: F, t118098: F, t118129: F, t121520: F, t122539: F, t122745: F, t122818: F, t2807: F, t33196: F, t34400: F, t34406: F, t34412: F, t35430: F, t9732: F) -> (F,) {
    let t122839 = t9740 * t4998 * t35382;
    let t122850 = t2804 * t122596;
    let t122852 = t34444 * t34484;
    let t122855 = -t118091 - t118098 + 0.22114583333333333334e-1 * t33196 * t122818 - 0.120625e-1 * t33196 * t122539 - 0.20104166666666666667e-2 * t33196 * t122745 + 0.11574074074074074074e-2 * t122839 + 0.27777777777777777778e-1 * t34412 * t34400 + 0.55555555555555555556e-1 * t34412 * t34406 - 0.52083333333333333333e-2 * t35430 * t9732 * t2807 - 0.51588271604938271603e-3 * t116723 + 0.34822083333333333332e-2 * t121520 - 0.34722222222222222223e-2 * t122850 + 0.13402777777777777778e-2 * t122852 - 0.77382407407407407407e-3 * t116731 - t118129;
    (t122855,)
}
