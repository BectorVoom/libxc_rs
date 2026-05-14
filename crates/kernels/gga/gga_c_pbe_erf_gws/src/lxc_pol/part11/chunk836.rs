//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 836/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk836<F: Float>(t1215: F, t457: F, t4619: F, t470: F, t1327: F, t1333: F, t1319: F, t1322: F, t18563: F, t18639: F, t456: F, t4605: F, t16576: F, t88: F, t1438: F, t461: F, t4862: F) -> (F, F, F, F, F, F, F) {
    let t18939 = 0.46785787179641632568e1 * t470 * t1215 * t4619 * t457;
    let t18941 = 120.0 * t1333 * t1327;
    let t18950 = 0.51947267698127589897e2 * t470 * t1319 * t18563 * t1322;
    let t18954 = 0.1403573615389248977e2 * t470 * t4605 * t18639 * t456;
    let t18955 = t16576 * t88;
    let t18956 = 384.0 * t18955;
    let t18958 = t1438 * t1327;
    let t18959 = 192.0 * t18958;
    let t18961 = 480.0 * t4862 * t461;
    (t18939, t18941, t18950, t18954, t18956, t18959, t18961)
}
