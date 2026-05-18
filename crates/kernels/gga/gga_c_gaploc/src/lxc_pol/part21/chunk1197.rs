//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1197/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1197<F: Float>(t32160: F, t1895: F, t481: F, t686: F, t10809: F, t169: F, t7305: F, t10704: F, t21665: F, t1843: F, t24474: F, t7064: F) -> (F, F, F, F, F) {
    let t32161 = F::new(0.85450291446024714264e-3) * t32160;
    let t32163 = t481 * t1895 * t686;
    let t32167 = F::new(0.1845726295234133828e0) * t32163 * t10809 * t169 * t7305;
    let t32168 = t21665 * t10704;
    let t32169 = F::new(0.64087718584518535698e-3) * t32168;
    let t32171 = t7064 * t1843 * t24474;
    (t32161, t32163, t32167, t32169, t32171)
}
