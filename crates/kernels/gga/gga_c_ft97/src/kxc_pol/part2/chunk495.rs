//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 495/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk495<F: Float>(t2832: F, t312: F, t2649: F, t2745: F, t2750: F, t2802: F, t2845: F, t2892: F, t301: F, t317: F, t830: F, t880: F) -> (F, F) {
    let t2894 = t2832 * t312;
    let t2899 = -t2649 * t317 - t2745 * t317 - t2892 * t301 - F::new(2.0) * t830 * t880 - F::new(4.0) * t2750 - F::new(2.0) * t2802 + F::new(4.0) * t2845 + F::new(2.0) * t2894;
    (t2894, t2899)
}
