//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 691/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk691<F: Float>(t1775: F, t3135: F, t3128: F, t2: F, t8275: F, t11175: F, t17: F, t9: F, t3141: F, t8282: F, t959: F, t3151: F) -> (F, F, F, F, F, F, F) {
    let t11684 = F::new(4.0) / F::new(9.0) * t1775 * t3135;
    let t11686 = F::new(4.0) / F::new(27.0) * t1775 * t3128;
    let t11690 = t8275 * t2;
    let t11717 = t9 * t11175 * t17;
    let t11718 = t11717 * t3141;
    let t11720 = t8282 * t959;
    let t11732 = F::new(4.0) / F::new(3.0) * t1775 * t3151;
    (t11684, t11686, t11690, t11717, t11718, t11720, t11732)
}
