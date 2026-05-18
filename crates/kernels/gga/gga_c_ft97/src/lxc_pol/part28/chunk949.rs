//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 949/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk949<F: Float>(t17839: F, t397: F, t22833: F, t5544: F, t22856: F, t7178: F, t22855: F, t7195: F, t5587: F, t32174: F, t5607: F, t3076: F, t38241: F, t39: F, t40: F) -> (F, F, F, F, F, F, F, F) {
    let t136866 = t17839 * t397;
    let t136870 = t22833 * t5544;
    let t136885 = F::new(0.17024962234567901235e-1) * t7178 * t22856;
    let t136891 = t7195 * t22855;
    let t136893 = F::new(0.75685073759570552987e-4) * t5587 * t136891;
    let t136898 = t32174 * t5607;
    let t136899 = t5587 * t136898;
    let t136903 = t3076 * t38241 * t39 * t40;
    (t136866, t136870, t136885, t136891, t136893, t136898, t136899, t136903)
}
