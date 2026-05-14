//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1344/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1344<F: Float>(t18984: F, t18990: F, t18995: F, t19013: F, t19032: F, t23741: F, t23752: F, t23753: F, t23759: F, t23761: F, t23763: F, t32133: F, t32134: F, t32139: F, t32202: F, t32207: F) -> (F,) {
    let t32959 = t18984 - t32133 - t18990 - t23741 - t32134 + t18995 + t32139 + t23752 + t19013 - t23753 - t23759 - t23761 - t23763 + t32202 + t19032 - t32207;
    (t32959,)
}
