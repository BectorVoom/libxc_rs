//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 731/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk731<F: Float>(t1882: F, t7669: F, t7674: F, t684: F, t7629: F, t10703: F, t6353: F, t6365: F, t840: F, t7672: F, t824: F, t2843: F, t875: F, t10697: F, t296: F, t10683: F, t319: F, t33830: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t34156 = 2.0 / 9.0 * t1882 * t7669;
    let t34158 = 2.0 / 9.0 * t1882 * t7674;
    let t34159 = t7629 * t684;
    let t34160 = t10703 * t34159;
    let t34164 = t840 * t6353 * t6365;
    let t34167 = t7672 * t824;
    let t34169 = t840 * t2843 * t34167;
    let t34172 = t7672 * t875;
    let t34173 = t10697 * t34172;
    let t34174 = t296 * t34173;
    let t34178 = t10683 * t319 * t33830;
    (t34156, t34158, t34159, t34160, t34164, t34167, t34169, t34172, t34173, t34174, t34178)
}
