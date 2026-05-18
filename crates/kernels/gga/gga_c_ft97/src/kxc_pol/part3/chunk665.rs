//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 665/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk665<F: Float>(t191: F, t2999: F, t26: F, t1771: F, t685: F, t322: F, t668: F, t17: F, t2346: F, t667: F, t113: F, t170: F, t7512: F) -> (F, F, F, F, F, F, F, F, F) {
    let t9555 = t2999 * t191;
    let t9556 = t26 * t9555;
    let t9557 = F::new(28.0) / F::new(27.0) * t9556;
    let t9558 = t1771 * t685;
    let t9567 = F::new(1.0) / t322 / t668;
    let t9568 = t17 * t9567;
    let t9570 = F::new(1.0) / t2346 / t667;
    let t9577 = F::new(1.0) / t2346 / t113;
    let t9606 = F::new(4.0) * t170 * t7512;
    (t9555, t9556, t9557, t9558, t9567, t9568, t9570, t9577, t9606)
}
