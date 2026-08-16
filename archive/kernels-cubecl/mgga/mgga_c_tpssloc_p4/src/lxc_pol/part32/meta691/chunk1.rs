//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2138/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2138<F: Float>(t26331: F, t26333: F, t90566: F, t1985: F, t22666: F, t28205: F, t7700: F, t90739: F, t28206: F, t6883: F, t1385: F, t1992: F, t22635: F, t3886: F, t6460: F) -> (F, F, F, F, F) {
    let t96854 = t26331 * t90566 * t26333;
    let t96857 = t1985 * t22666 * t28205;
    let t96866 = t1985 * t90739 * t7700;
    let t96868 = t6883 * t28206;
    let t96873 = t1992 * t22635 * t3886 * t6460 * t1385;
    (t96854, t96857, t96866, t96868, t96873)
}
