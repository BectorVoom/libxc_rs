//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1211/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1211<F: Float>(t25260: F, t2558: F, t9647: F, t10691: F, t1850: F, t1895: F, t481: F, t686: F, t10809: F, t169: F, t7305: F, t10704: F, t21665: F) -> (F, F, F, F, F) {
    let t32158 = t9647 * t25260 * t2558;
    let t32159 = F::cast_from(0.32043859292259267849e-3_f64) * t32158;
    let t32160 = t1850 * t10691;
    let t32161 = F::cast_from(0.85450291446024714264e-3_f64) * t32160;
    let t32163 = t481 * t1895 * t686;
    let t32167 = F::cast_from(0.1845726295234133828e0_f64) * t32163 * t10809 * t169 * t7305;
    let t32168 = t21665 * t10704;
    (t32159, t32161, t32163, t32167, t32168)
}
