//! GGA_C_GAPC lxc pol — lxc_pol part 27 (v4rho2sigma2_6) CSE chunk 1196/1310 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part27_v4rho2sigma2_6_chunk1196<F: Float>(t11587: F, t11591: F, t3060: F, t28006: F, t3112: F, t33498: F, t8362: F, t11488: F, t1688: F, t21157: F, t1743: F, t33958: F, t34711: F) -> (F, F, F, F) {
    let t34772 = t3060 * t11587 * t11591;
    let t34776 = t3112 * t33498 * t8362 * t28006;
    let t34779 = t11488 * t1688 * t21157;
    let t34782 = t1743 * t33958 * t34711;
    (t34772, t34776, t34779, t34782)
}
