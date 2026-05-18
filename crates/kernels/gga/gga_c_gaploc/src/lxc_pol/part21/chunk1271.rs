//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1271/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1271<F: Float>(t33568: F, t10847: F, t22693: F, t7572: F, t24554: F, t959: F, t20671: F, t22538: F, t24549: F, t11057: F, t28737: F, t10942: F, t28673: F) -> (F, F, F, F, F, F) {
    let t33569 = F::new(0.29792074959875355558e-1) * t33568;
    let t33572 = F::new(0.18404604457881959845e2) * t7572 * t22693 * t10847;
    let t33573 = t24554 * t959;
    let t33574 = F::new(0.14896037479937677779e-1) * t33573;
    let t33580 = t22538 * t20671 * t24549;
    let t33581 = F::new(0.85206502119823888168e-1) * t33580;
    let t33583 = t28737 * t11057;
    let t33584 = F::new(0.76685851907841499352e0) * t33583;
    let t33585 = t28673 * t10942;
    (t33569, t33572, t33574, t33581, t33584, t33585)
}
