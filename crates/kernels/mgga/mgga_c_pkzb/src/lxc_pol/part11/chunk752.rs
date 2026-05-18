//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 752/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk752<F: Float>(t2241: F, t351: F, t6087: F, t6174: F, t2316: F, t880: F) -> (F, F, F, F) {
    let t6201 = F::new(1.0) / t2241 / t351;
    let t6211 = F::new(0.93932222222222222223e0) * t6087;
    let t6218 = F::new(0.36793333333333333333e0) * t6174;
    let t6230 = F::new(1.0) / t2316 / t880;
    (t6201, t6211, t6218, t6230)
}
