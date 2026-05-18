//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1123/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1123<F: Float>(t2228: F, t2242: F, t6751: F, t6832: F, t375: F, t6125: F, t2417: F, t6336: F, t6707: F, t4379: F, t6: F, t6322: F, t6563: F) -> (F, F, F, F, F, F, F) {
    let t20160 = t2242 * t2228;
    let t20162 = t6832 * t6751;
    let t20173 = F::new(1.0) / t6125 / t375;
    let t20174 = t2417 * t2417;
    let t20181 = t6336 * t6707 / F::new(24.0);
    let t20182 = t6 * t4379;
    let t20188 = F::new(3.0) / F::new(8.0) * t6322 * t6563;
    (t20160, t20162, t20173, t20174, t20181, t20182, t20188)
}
