//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1239/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1239<F: Float>(t2423: F, t18899: F, t18961: F, t18964: F, t18968: F, t18970: F, t18973: F, t18975: F, t18977: F, t18979: F, t18981: F, t18983: F, t2053: F, t2054: F, t2074: F, t2075: F, t20988: F, t2182: F, t2429: F, t321: F, t6855: F, t804: F, t810: F, t8524: F) -> F {
    let t21890 = t2423 * t2423;
    let t21905 = -F::new(3.0) * t2053 * t21890 * t321 - F::new(18.0) * t2054 * t2074 * t804 - F::new(36.0) * t2054 * t2182 * t2429 + F::new(24.0) * t6855 * t804 * t810 + F::new(36.0) * t2075 * t8524 - t18899 - t18961 + t18964 - t18968 + t18970 - t18973 + t18975 - t18977 + t18979 + t18981 - t18983 - t20988;
    t21905
}
