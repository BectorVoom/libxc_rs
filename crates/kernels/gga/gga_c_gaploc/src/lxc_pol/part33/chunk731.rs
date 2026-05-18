//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 731/1464 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk731<F: Float>(t6895: F, t912: F, t587: F, t2478: F, t589: F, t2476: F, t549: F, t6536: F, t161: F, t4774: F) -> (F, F, F, F) {
    let t6896 = t912 * t6895;
    let t6897 = t587 * t6896;
    let t6899 = t589 * t2478;
    let t6900 = t2476 * t6899;
    let t6904 = t549 * t6536;
    let t6907 = t161 * t4774;
    (t6897, t6900, t6904, t6907)
}
