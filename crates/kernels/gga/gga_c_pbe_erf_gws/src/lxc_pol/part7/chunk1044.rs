//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1044/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1044<F: Float>(t1327: F, t1333: F, t40: F, t460: F, t4778: F, t1423: F, t1319: F, t1322: F, t18563: F, t470: F, t18639: F, t456: F, t4605: F) -> (F, F, F, F, F) {
    let t18941 = F::cast_from(120.0_f64) * t1333 * t1327;
    let t18943 = t40 * t4778 * t460;
    let t18944 = F::cast_from(4.0_f64) * t18943;
    let t18945 = t1333 * t1423;
    let t18946 = F::cast_from(120.0_f64) * t18945;
    let t18950 = F::cast_from(0.51947267698127589897e2_f64) * t470 * t1319 * t18563 * t1322;
    let t18954 = F::cast_from(0.1403573615389248977e2_f64) * t470 * t4605 * t18639 * t456;
    (t18941, t18944, t18946, t18950, t18954)
}
