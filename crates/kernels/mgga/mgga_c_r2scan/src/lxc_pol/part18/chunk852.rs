//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 852/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk852<F: Float>(t3034: F, t725: F, t41: F, t5812: F, t5815: F, t5818: F, t5821: F, t5925: F, t5936: F, t5940: F, t5945: F, t5950: F, t5959: F, t5963: F) -> F {
    let t9014 = t3034 * t725;
    let t9015 = t41 * t9014;
    let t9017 = t5812 + t5815 + t5925 - t9015 - t5818 + t5821 + F::new(0.72290542002011598948e-2) * t5936 + t5940 + t5945 - t5950 + t5959 + t5963;
    t9017
}
