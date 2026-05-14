//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 907/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk907<F: Float>(t12549: F, t1651: F, t587: F, t12531: F, t1620: F, t5493: F, t10913: F, t7130: F, t12659: F, t1820: F, t5018: F, t3346: F, t995: F, t1022: F, t3354: F, t12829: F, t679: F) -> (F, F, F, F, F, F, F) {
    let t41056 = t587 * t1651 * t12549;
    let t41061 = t1620 * t5493 * t12531;
    let t41069 = t7130 * t10913;
    let t41074 = t1820 * t5018 * t12659;
    let t41095 = t3346 * t995;
    let t41133 = t3354 * t1022;
    let t41184 = t12829 * t679;
    (t41056, t41061, t41069, t41074, t41095, t41133, t41184)
}
