//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 815/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk815<F: Float>(t108530: F, t505: F, t65692: F, t695: F, t1103: F, t1614: F, t17836: F, t52: F, t6018: F, t1100: F, t13442: F, t6776: F, t236: F, t2426: F, t2427: F, t3758: F) -> (F, F, F, F, F, F, F, F, F) {
    let t108531 = t108530 * t505;
    let t108585 = t65692 * t695;
    let t108795 = t1614 * t1103;
    let t108826 = t17836 * t6018 * t52;
    let t108897 = t1100 * t13442;
    let t109108 = t695 * t6776;
    let t109200 = t236 * t6776;
    let t109216 = t2426 * t6776;
    let t109230 = t3758 * t2427;
    (t108531, t108585, t108795, t108826, t108897, t109108, t109200, t109216, t109230)
}
