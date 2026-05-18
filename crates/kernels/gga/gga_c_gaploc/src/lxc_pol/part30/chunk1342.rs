//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1342/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1342<F: Float>(t10624: F, t1382: F, t605: F, t7329: F, t8862: F, t1960: F, t8854: F, t977: F, t1959: F, t3455: F, t2497: F, t2902: F) -> (F, F, F, F, F) {
    let t33986 = F::new(4.0) * t1382 * t10624 * t605;
    let t33988 = F::new(4.0) * t8862 * t7329;
    let t33991 = F::new(2.0) * t1960 * t8854 * t977;
    let t33992 = t3455 * t1959;
    let t33997 = F::new(4.0) * t1382 * t2902 * t2497;
    (t33986, t33988, t33991, t33992, t33997)
}
