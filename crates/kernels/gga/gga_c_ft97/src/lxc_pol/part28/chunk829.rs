//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 829/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk829<F: Float>(t33049: F, t33130: F, t33174: F, t33219: F, t160: F, t33119: F, t1349: F, t149: F, t32743: F, t32750: F, t32872: F, t32876: F, t32881: F, t32993: F, t32995: F, t32998: F, t33002: F, t33045: F, t33086: F, t33091: F, t33179: F, t5766: F, t5772: F, t5781: F, t5849: F, t7309: F, t7315: F, t7342: F) -> (F, F, F) {
    let t33221 = t33049 + t33130 + t33174 + t33219;
    let t33227 = t33119 * t160;
    let t33229 = -t1349 * t32743 / F::new(3.0) + t5766 * t7342 / F::new(6.0) + t32750 + t1349 * t32872 / F::new(6.0) + t1349 * t32876 / F::new(6.0) - t5772 * t32881 / F::new(9.0) - t5766 * t7315 / F::new(3.0) + t7309 * t5849 / F::new(6.0) - F::new(2.0) * t32993 + F::new(4.0) * t32995 + t1349 * t32998 - F::new(2.0) / F::new(3.0) * t1349 * t33002 - t7309 * t5781 / F::new(3.0) - t149 * t33221 + F::new(8.0) * t33179 + F::new(4.0) * t33045 - F::new(12.0) * t33086 + F::new(8.0) * t33091 + F::new(2.0) * t33227;
    (t33221, t33227, t33229)
}
