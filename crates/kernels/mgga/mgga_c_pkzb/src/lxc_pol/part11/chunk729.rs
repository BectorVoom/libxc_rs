//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 729/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk729<F: Float>(t1485: F, t178: F, t301: F, t299: F, t2003: F, t53: F, t2002: F, t208: F) -> (F, F, F, F) {
    let t5612 = t178 * t1485 * t301;
    let t5614 = F::new(0.63517063878621832551e-4) * t299 * t5612;
    let t5627 = t53 * t2003;
    let t5633 = F::new(1.0) / t2002 / t208;
    (t5612, t5614, t5627, t5633)
}
