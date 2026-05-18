//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 977/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk977<F: Float>(t31867: F, t2138: F, t2147: F, t463: F, t8064: F, t1265: F, t8331: F, t2132: F, t3037: F, t32146: F, t633: F, t7885: F, t8336: F) -> (F, F, F, F, F) {
    let t32967 = F::new(0.2767432121485165382e-1) * t31867;
    let t32990 = t2138 * t2147 * t8064 * t463;
    let t32992 = t8331 * t1265;
    let t32997 = F::new(0.10408353825846239354e2) * t32146 * t2132 * t633 * t3037;
    let t33000 = t7885 * t2147 * t8336 * t463;
    (t32967, t32990, t32992, t32997, t33000)
}
