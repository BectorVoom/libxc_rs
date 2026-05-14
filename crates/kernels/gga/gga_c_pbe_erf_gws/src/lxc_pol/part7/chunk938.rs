//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 938/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk938<F: Float>(t16576: F, t88: F, t18853: F, t18863: F, t18928: F, t18933: F, t18935: F, t18939: F, t18941: F, t18944: F, t18946: F, t18950: F, t18954: F, t1327: F, t1438: F, t461: F, t4862: F) -> (F, F, F, F) {
    let t18955 = t16576 * t88;
    let t18956 = 384.0 * t18955;
    let t18957 = t18853 - t18863 + t18928 - t18933 + t18935 + t18939 + t18941 + t18944 + t18946 - t18950 + t18954 - t18956;
    let t18958 = t1438 * t1327;
    let t18959 = 192.0 * t18958;
    let t18961 = 480.0 * t4862 * t461;
    (t18956, t18957, t18959, t18961)
}
