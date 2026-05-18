//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1164/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1164<F: Float>(t245: F, t1178: F, t20044: F, t21: F, t21780: F, t267: F, t4431: F, t5: F, t5186: F, t85501: F, t89629: F, t89749: F, t920: F) -> F {
    let t246 = F::new(10000000.0) <= t245;
    let t89765 = piecewise3::<f64>(t246, F::new(0.0), t5 * (t89629 + t89749) * t21 / F::new(4.0) + t5 * t21780 * t920 + F::new(3.0) / F::new(2.0) * t5 * t5186 * t4431 + t5 * t1178 * t20044 + t5 * t267 * t85501 / F::new(4.0));
    t89765
}
