//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1101/1268 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1101<F: Float>(t10737: F, t5293: F, t10731: F, t7137: F, t1841: F, t7222: F, t8878: F, t1022: F, t2530: F) -> (F, F, F, F) {
    let t32429 = 0.41016139894091862846e-1 * t5293 * t10737;
    let t32431 = 0.24609683936455117708e0 * t7137 * t10731;
    let t32434 = 0.51270174867614828558e-2 * t1841 * t8878 * t7222;
    let t32435 = t1022 * t2530;
    (t32429, t32431, t32434, t32435)
}
