//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1014/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1014<F: Float>(t10972: F, t2790: F, t10326: F, t11037: F, t2615: F, t12549: F, t1651: F, t587: F, t12531: F, t1620: F, t5493: F, t10913: F, t7130: F) -> (F, F, F, F, F, F) {
    let t41046 = t2790 * t10972;
    let t41048 = t10326 * t10972;
    let t41053 = t2615 * t11037;
    let t41056 = t587 * t1651 * t12549;
    let t41061 = t1620 * t5493 * t12531;
    let t41069 = t7130 * t10913;
    (t41046, t41048, t41053, t41056, t41061, t41069)
}
