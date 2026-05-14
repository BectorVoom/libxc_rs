//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 579/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk579<F: Float>(t1595: F, t1597: F, t63: F, t7857: F, t39: F, t409: F, t64: F, t25: F, t1602: F, t35: F, t401: F, t428: F, t1681: F, t53: F, t1711: F, t6: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t7858 = t1595 * t1597;
    let t7859 = t7858 * t63;
    let t7860 = t7857 * t7859;
    let t7866 = t409 * t39;
    let t7867 = t64 * t7866;
    let t7876 = t409 * t25;
    let t7877 = t1602 * t7876;
    let t7878 = t35 * t401;
    let t7879 = t7878 * t428;
    let t7883 = t53 * t1681;
    let t7888 = t1711 * t6;
    (t7858, t7860, t7866, t7867, t7876, t7877, t7878, t7879, t7883, t7888)
}
