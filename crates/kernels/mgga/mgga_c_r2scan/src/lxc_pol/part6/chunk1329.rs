//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1329/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1329<F: Float>(t19620: F, t19624: F, t19628: F, t19646: F, t19649: F, t19687: F, t19728: F, t19748: F, t23951: F, t23954: F, t23956: F, t23959: F, t23961: F, t23970: F, t23972: F, t23980: F) -> (F,) {
    let t25030 = -t19620 + t19624 - t19628 - t19646 - t19649 - t19728 + t19687 + t23951 - t23954 + t23956 - t23959 - t23961 + t23970 - t23972 - t19748 - t23980;
    (t25030,)
}
