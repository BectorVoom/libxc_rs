//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1144/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1144<F: Float>(t2559: F, t47442: F, t587: F, t12613: F, t7527: F, t1620: F, t1809: F, t32114: F, t3351: F, t48169: F, t48173: F, t48175: F, t48179: F, t48183: F, t48187: F, t48191: F, t48195: F, t48198: F) -> (F, F, F, F) {
    let t48201 = F::new(16.0) / F::new(3.0) * t587 * t2559 * t47442;
    let t48203 = F::new(32.0) / F::new(15.0) * t7527 * t12613;
    let t48207 = F::new(32.0) / F::new(15.0) * t1620 * t1809 * t32114 * t3351;
    let t48208 = t48169 + t48173 - t48175 - t48179 + t48183 + t48187 - t48191 - t48195 - t48198 - t48201 + t48203 - t48207;
    (t48201, t48203, t48207, t48208)
}
