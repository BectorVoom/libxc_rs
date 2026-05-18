//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 868/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk868<F: Float>(t2781: F, t35863: F, t1486: F, t193: F, t1208: F, t230: F, t420: F, t7470: F, t1196: F, t287: F, t35462: F, t290: F) -> (F, F, F, F, F, F, F) {
    let t35864 = t2781 * t35863;
    let t35866 = t1486 * t193 * t35864;
    let t35870 = t230 * t1208;
    let t35871 = t420 * t35870;
    let t35872 = t7470 * t35871;
    let t35877 = t230 * t1196;
    let t35878 = t420 * t35877;
    let t35879 = t7470 * t35878;
    let t35886 = t35462 * t287;
    let t35887 = t35886 * t290;
    (t35864, t35866, t35870, t35872, t35877, t35879, t35887)
}
