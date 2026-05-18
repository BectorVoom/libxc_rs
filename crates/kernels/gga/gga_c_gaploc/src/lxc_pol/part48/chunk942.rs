//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 942/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk942<F: Float>(t2358: F, t35770: F, t3684: F, t7822: F, t11125: F, t2969: F, t3511: F, t45163: F, t45164: F, t45202: F, t45257: F, t45309: F, t45363: F, t45412: F, t45455: F, t45512: F, t45570: F, t45619: F, t45672: F, t45718: F, t45767: F, t45824: F, t45875: F, t45919: F, t45959: F, t45967: F, t45969: F, t45971: F, t45973: F, t45974: F, t45976: F, t45978: F, t45983: F, t45986: F, t45988: F, t45990: F, t748: F, t8440: F) -> (F, F) {
    let t45992 = F::new(2.0) * t35770 * t2358;
    let t45993 = t7822 * t3684;
    let t45994 = -t45163 + t45164 - F::new(2.0) * t2969 * t11125 - t748 * (t45202 + t45257 + t45309 + t45363 + t45412 + t45455 + t45512 + t45570 + t45619 + t45672 + t45718 + t45767 + t45824 + t45875 + t45919 + t45959) - t45967 - t45969 - t45971 - t45973 + t45974 + t45976 + t45978 - F::new(2.0) * t8440 * t3511 + t45983 - t45986 + t45988 + t45990 - t45992 - t45993;
    (t45992, t45994)
}
