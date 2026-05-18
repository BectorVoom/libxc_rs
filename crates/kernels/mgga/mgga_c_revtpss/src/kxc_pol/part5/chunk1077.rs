//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1077/1422 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk1077<F: Float>(t14600: F, t676: F, t836: F, t14598: F, t1558: F, t879: F, t2482: F, t2801: F, t1531: F, t37: F, t4392: F, t72: F) -> (F, F, F, F) {
    let t14602 = t14600 * t676 * t836;
    let t14603 = t14598 * t14602;
    let t14605 = t879 * t1558;
    let t14606 = t2482 * t14605;
    let t14608 = F::new(0.19514881078765566038e-1) * t14606 * t2801;
    let t14613 = t37 * t1531;
    let t14616 = t4392 * t72;
    (t14603, t14608, t14613, t14616)
}
