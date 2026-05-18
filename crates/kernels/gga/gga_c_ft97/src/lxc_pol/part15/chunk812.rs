//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 812/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk812<F: Float>(t21204: F, t4342: F, t14431: F, t14445: F, t18823: F, t18825: F, t18874: F, t21871: F, t21875: F, t21878: F, t21881: F, t21885: F, t21893: F, t2265: F, t631: F) -> (F, F) {
    let t21895 = t4342 * t21204;
    let t21897 = t2265 * t21871 / F::new(6.0) - t2265 * t21875 - t2265 * t21878 + F::new(3.0) * t2265 * t21881 + F::new(2.0) * t2265 * t21885 + F::new(5.0) / F::new(9.0) * t14431 - t18823 + F::new(3.0) * t18825 + F::new(5.0) / F::new(3.0) * t14445 - t18874 / F::new(9.0) + t631 * t21893 - t2265 * t21895;
    (t21895, t21897)
}
