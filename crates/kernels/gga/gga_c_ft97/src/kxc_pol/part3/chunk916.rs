//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 916/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk916<F: Float>(t238: F, t17876: F, t17931: F, t17992: F, t18136: F, t676: F, t27: F, t89: F, t375: F, t4935: F, t5054: F, t2371: F, t5053: F) -> (F, F, F, F, F) {
    let t239 = F::new(0.1e-59) < t238;
    let t18139 = piecewise3::<F>(t239, t17876 + t17931 + t17992 + t18136, F::new(0.0));
    let t18140 = t676 * t18139;
    let t18142 = t89 * t27 * t18140;
    let t18145 = t89 * t375 * t4935;
    let t18148 = t89 * t375 * t5054;
    let t18150 = t2371 * t5053;
    (t18139, t18142, t18145, t18148, t18150)
}
