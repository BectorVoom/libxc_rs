//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1048/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1048<F: Float>(t1: F, t2049: F, t2057: F, t2062: F, t18885: F, t18899: F, t18959: F, t18961: F, t18964: F, t18968: F, t18970: F, t18973: F, t18975: F, t18977: F, t18979: F, t18981: F, t18983: F) -> (F, F) {
    let t20987 = t2049 * t2057 * t1 * t2062;
    let t20988 = 0.37963457796989083263e1 * t20987;
    let t20989 = t18885 - t18959 - t18961 + t18964 - t18968 + t18970 - t18973 + t18975 - t18977 + t18979 + t18981 - t20988 - t18983 - t18899;
    (t20988, t20989)
}
