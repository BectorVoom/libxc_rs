//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 688/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk688<F: Float>(t2253: F, t2930: F, t703: F, t900: F, t230: F, t2938: F, t9556: F, t2937: F, t325: F, t8640: F, t895: F, t2934: F) -> (F, F, F, F, F, F, F) {
    let t10843 = t2253 * t2930;
    let t10845 = t703 * t900;
    let t10864 = t230 * t2938;
    let t10883 = F::cast_from(0.44934037037037037036e0_f64) * t9556;
    let t10904 = F::new(1.0) / t2937 / t325;
    let t10921 = t8640 * t895;
    let t10923 = t2253 * t2934;
    (t10843, t10845, t10864, t10883, t10904, t10921, t10923)
}
