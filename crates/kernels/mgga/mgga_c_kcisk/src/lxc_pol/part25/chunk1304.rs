//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1304/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1304<F: Float>(t15892: F, t1869: F, t33017: F, t4803: F, t112269: F, t116683: F, t116687: F, t116690: F, t116695: F, t116698: F, t116701: F, t116703: F, t116705: F, t17269: F, t32913: F, t33005: F, t34013: F, t34073: F, t34122: F, t34225: F, t7261: F, t9664: F, t9670: F) -> (F, F) {
    let t116710 = t1869 * t33017 * t15892 * t4803;
    let t116718 = 0.26805555555555555556e-2 * t112269 * t34013 - 0.11054629629629629629e-2 * t116683 - 0.55273148148148148147e-3 * t116687 + 0.27636574074074074073e-2 * t116690 + 0.32166666666666666669e-1 * t34225 * t33005 - 0.44218518518518518517e-2 * t116695 - 0.73697530864197530862e-2 * t116698 + t116701 + t116703 + t116705 + 0.10416666666666666667e-1 * t34073 * t32913 - 0.16581944444444444444e-2 * t116710 + 0.10416666666666666667e-1 * t34122 * t32913 + 0.10416666666666666667e-1 * t9664 * t7261 * t9670 * t17269;
    (t116710, t116718)
}
