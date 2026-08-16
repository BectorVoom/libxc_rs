//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 809/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk809<F: Float>(t7672: F, t875: F, t10697: F, t296: F, t10683: F, t319: F, t33830: F, t7584: F, t2862: F, t871: F, t33835: F, t1882: F, t7635: F) -> (F, F, F, F, F, F, F, F) {
    let t34172 = t7672 * t875;
    let t34173 = t10697 * t34172;
    let t34174 = t296 * t34173;
    let t34178 = t10683 * t319 * t33830;
    let t34181 = t7584 * t875;
    let t34183 = t2862 * t871 * t34181;
    let t34187 = t2862 * t319 * t33835;
    let t34191 = t1882 * t7635 / F::cast_from(9.0_f64);
    (t34172, t34173, t34174, t34178, t34181, t34183, t34187, t34191)
}
