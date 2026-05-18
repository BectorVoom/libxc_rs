//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 253/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk253<F: Float>(t301: F, t317: F, t830: F, t876: F, t880: F, t882: F, t332: F, t321: F, t5: F, t170: F, t328: F, t626: F) -> (F, F, F, F) {
    let t885 = -t301 * t880 - t317 * t830 - F::new(2.0) * t876 + F::new(2.0) * t882;
    let t886 = t885 * t332;
    let t889 = t5 * t321;
    let t892 = t170 * t626 * t328 / F::new(6.0);
    (t885, t886, t889, t892)
}
