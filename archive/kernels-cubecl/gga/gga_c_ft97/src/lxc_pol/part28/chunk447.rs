//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 447/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk447<F: Float>(t1322: F, t461: F, t1307: F, t72: F, t2: F, t342: F, t343: F, t7151: F, t4: F, t26: F) -> (F, F, F, F) {
    let t7152 = t461 * t1322;
    let t7155 = t72 * t1307;
    let t7160 = (-t7151 * t7152 / F::cast_from(6.0_f64) - t342 * t343 * t7155 / F::cast_from(4.0_f64)) * t2;
    let t7161 = t7160 * t4;
    let t7162 = t7161 * t26;
    (t7152, t7155, t7161, t7162)
}
