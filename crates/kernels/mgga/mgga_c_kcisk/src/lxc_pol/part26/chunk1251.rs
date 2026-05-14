//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1251/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1251<F: Float>(t110815: F, t110817: F, t110907: F, t110912: F, t110956: F, t110992: F, t111025: F, t111059: F, t111090: F, t111121: F, t111151: F, t111187: F, t111193: F, t15716: F, t15724: F, t15760: F, t2705: F, t289: F, t32579: F, t32584: F, t3437: F, t3442: F, t3460: F, t43179: F, t9404: F) -> (F,) {
    let t111196 = -t110815 + 2.0 * t3442 * t2705 * t15760 + t110817 - t110907 - 3.0 * t3437 * t32579 + t110912 + 6.0 * t3442 * t9404 * t3460 + 12.0 * t15716 * t32584 + (t110956 + t110992 + t111025 + t111059 + t111090 + t111121 + t111151 + t111187) * t289 - t43179 * t2705 - 6.0 * t111193 * t15724;
    (t111196,)
}
