//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 683/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk683<F: Float>(t1790: F, t7283: F, t1693: F, t1792: F, t2470: F, t4812: F, t4814: F, t4823: F, t4830: F, t5075: F, t671: F, t6884: F, t6959: F, t6963: F, t6968: F, t6971: F, t6976: F, t6979: F, t6983: F, t6988: F, t6990: F, t6992: F, t7072: F, t7275: F, t7278: F) -> (F, F) {
    let t7284 = t7283 * t1790;
    let t7290 = -0.16581944444444444444e-2 * t4812 + 0.11054629629629629629e-2 * t4814 + 0.11054629629629629629e-2 * t6959 + 0.66327777777777777776e-2 * t6963 - 0.44218518518518518517e-2 * t6968 + 0.16581944444444444444e-2 * t6971 - 0.24872916666666666666e-2 * t6976 + 0.16581944444444444444e-2 * t6979 + 0.11054629629629629629e-2 * t6983 - 0.16581944444444444444e-2 * t6988 - 0.44218518518518518517e-2 * t6990 + 0.16581944444444444444e-2 * t6992 + 0.11054629629629629629e-2 * t5075 - 0.24872916666666666666e-2 * t7072 - 0.193e0 * t1693 * t7275 - 0.193e0 * t7278 * t1792 - 0.193e0 * t4830 * t2470 + 0.193e0 * t1693 * t7284 + 0.74498e-1 * t4823 * t7284 + t6884 * t671;
    (t7284, t7290)
}
