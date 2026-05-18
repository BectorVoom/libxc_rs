//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1160/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1160<F: Float>(t66903: F, t66906: F, t66935: F, t66946: F, t67421: F, t68751: F, t68774: F, t80685: F, t80696: F, t80759: F, t88186: F, t88190: F, t88198: F, t88201: F) -> F {
    let t89712 = F::new(4.0) / F::new(3.0) * t88186 + F::new(2.0) / F::new(9.0) * t88190 + F::new(4.0) / F::new(3.0) * t80685 - t66903 + t66906 + F::new(4.0) / F::new(9.0) * t88198 - F::new(4.0) / F::new(3.0) * t88201 + F::new(4.0) / F::new(9.0) * t80696 + t66935 - t66946 + t68751 + t68774 - F::new(8.0) / F::new(27.0) * t80759 - t67421;
    t89712
}
