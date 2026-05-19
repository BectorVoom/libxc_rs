//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1416/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1416<F: Float>(t12210: F, t2009: F, t2066: F, t32778: F, t32785: F, t32791: F, t32806: F, t32813: F, t32815: F, t32818: F, t32821: F, t32824: F, t32827: F, t32829: F, t32832: F, t32835: F, t32839: F, t3732: F, t5724: F) -> F {
    let t38930 = -t32778 - t32785 - t32791 - t32806 + t32813 + t32815 - t32818 - t32821 + t32824 + t32827 - F::cast_from(0.35750489951850426669e0_f64) * t12210 * t5724 - F::cast_from(0.71500979903700853338e0_f64) * t2066 * t3732 * t2009 - t32829 - t32832 + t32835 - t32839;
    t38930
}
