//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 940/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk940<F: Float>(t428: F, t4358: F, t1336: F, t1423: F, t4862: F, t18885: F, t18899: F, t18959: F, t18961: F, t18964: F, t18968: F, t18970: F, t18973: F, t18975: F, t18977: F, t18463: F, t18532: F, t18584: F, t18620: F, t18663: F, t18925: F, t18957: F) -> (F, F, F, F) {
    let t18978 = t4358 * t428;
    let t18979 = 96.0 * t18978;
    let t18980 = t1336 * t1423;
    let t18981 = 72.0 * t18980;
    let t18982 = t4862 * t428;
    let t18983 = 480.0 * t18982;
    let t18984 = t18885 - t18959 - t18961 + t18964 - t18968 + t18970 - t18973 + t18975 - t18977 + t18979 + t18981 - t18983 - t18899;
    let t18987 = t18463 + t18532 + t18584 + t18620 + t18663 + t18925 + t18957 + t18984;
    (t18979, t18981, t18983, t18987)
}
