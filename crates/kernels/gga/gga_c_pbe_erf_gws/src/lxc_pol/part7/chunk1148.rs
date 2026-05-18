//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1148/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1148<F: Float>(t6658: F, t825: F, t337: F, t5: F, t6385: F, t2146: F, t6319: F, t6707: F, t6257: F, t6331: F, t2112: F, t4394: F) -> (F, F, F, F) {
    let t20560 = t825 * t6658;
    let t20563 = t20560 * t337 * t5 * t6385;
    let t20564 = t2146 * t20563;
    let t20566 = t6319 * t6707 / F::new(32.0);
    let t20567 = t6331 * t6257;
    let t20568 = t2146 * t20567;
    let t20569 = F::new(7.0) / F::new(12.0) * t20568;
    let t20571 = t2112 * t4394;
    (t20564, t20566, t20569, t20571)
}
