//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 678/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk678<F: Float>(t2336: F, t2675: F, t89: F, t2661: F, t9725: F, t272: F, t9606: F, t2417: F, t274: F, t668: F, t505: F, t123: F, t805: F) -> (F, F, F, F, F, F) {
    let t10282 = t89 * t2336 * t2675;
    let t10286 = t89 * t9725 * t2661;
    let t10304 = F::cast_from(1.0_f64) / t272 / t9606;
    let t10309 = t274 * t2417;
    let t10327 = t274 * t668;
    let t10328 = t10327 * t505;
    let t10339 = t123 / t805 / t9606;
    (t10282, t10286, t10304, t10309, t10328, t10339)
}
