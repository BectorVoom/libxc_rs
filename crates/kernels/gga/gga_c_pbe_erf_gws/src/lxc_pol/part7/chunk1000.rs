//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1000/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1000<F: Float>(t6751: F, t6832: F, t375: F, t6125: F, t2417: F, t6336: F, t6707: F, t4379: F, t6: F, t6322: F, t6563: F, t4422: F, t828: F, t2123: F, t2120: F, t6203: F, t6208: F) -> (F, F, F, F, F, F, F, F, F) {
    let t20162 = t6832 * t6751;
    let t20173 = 1.0 / t6125 / t375;
    let t20174 = t2417 * t2417;
    let t20181 = t6336 * t6707 / 24.0;
    let t20182 = t6 * t4379;
    let t20188 = 3.0 / 8.0 * t6322 * t6563;
    let t20189 = t4422 * t828;
    let t20190 = t20189 * t2123;
    let t20191 = t2120 * t20190;
    let t20192 = 35.0 / 72.0 * t20191;
    let t20193 = t6203 * t6208;
    (t20162, t20173, t20174, t20181, t20182, t20188, t20189, t20192, t20193)
}
