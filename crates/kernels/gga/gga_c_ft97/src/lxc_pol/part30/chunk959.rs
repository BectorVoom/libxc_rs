//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 959/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk959<F: Float>(t2843: F, t4299: F, t7679: F, t1466: F, t36056: F, t681: F, t28729: F, t33961: F, t143040: F, t143158: F, t35819: F, t684: F, t24980: F, t24981: F, t35853: F, t35833: F) -> (F, F, F, F, F, F, F, F) {
    let t152648 = t2843 * t7679 * t4299;
    let t152651 = t1466 * t681 * t36056;
    let t152657 = t33961 * t28729;
    let t152659 = t143040 * t143158 * t152657;
    let t152661 = t35819 * t684;
    let t152663 = t143040 * t143158 * t152661;
    let t152667 = t24980 * t24981 * t35853 * t684;
    let t152669 = t35833 * t684;
    (t152648, t152651, t152657, t152659, t152661, t152663, t152667, t152669)
}
