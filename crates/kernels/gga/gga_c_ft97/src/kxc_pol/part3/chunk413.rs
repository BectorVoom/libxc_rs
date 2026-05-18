//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 413/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk413<F: Float>(t5: F, t885: F, t170: F, t2248: F, t328: F, t2253: F, t895: F, t906: F, t70: F, t703: F, t2347: F, t327: F) -> (F, F, F, F, F, F) {
    let t2904 = t5 * t885;
    let t2912 = F::new(5.0) / F::new(18.0) * t170 * t2248 * t328;
    let t2913 = t2253 * t895;
    let t2915 = t2253 * t906;
    let t2917 = t70 * t703;
    let t2918 = t327 * t2347;
    (t2904, t2912, t2913, t2915, t2917, t2918)
}
