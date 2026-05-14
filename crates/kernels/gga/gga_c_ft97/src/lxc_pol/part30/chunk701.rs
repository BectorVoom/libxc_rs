//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 701/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk701<F: Float>(t33766: F, t729: F, t762: F, t258: F, t7440: F, t684: F, t10079: F, t24793: F, t6162: F, t242: F, t33596: F, t1424: F, t6187: F, t1901: F, t33743: F, t33747: F, t33748: F, t33751: F, t33756: F, t33761: F, t33765: F, t446: F) -> (F, F, F, F, F, F, F, F, F) {
    let t33768 = t729 * t762 * t33766;
    let t33771 = t258 * t7440;
    let t33772 = t33771 * t684;
    let t33773 = t10079 * t33772;
    let t33776 = t24793 * t6162;
    let t33779 = t242 * t33596;
    let t33782 = t1424 * t6187;
    let t33784 = t729 * t762 * t33782;
    let t33787 = -t446 * t33743 / 3.0 + t33747 - t446 * t33748 / 3.0 - 2.0 / 3.0 * t446 * t33751 + t1901 * t33756 / 9.0 - 2.0 / 9.0 * t1901 * t33761 - t33765 + t446 * t33768 / 3.0 - 2.0 / 9.0 * t1901 * t33773 + 2.0 / 9.0 * t1901 * t33776 + 2.0 / 3.0 * t446 * t33779 + 2.0 / 3.0 * t446 * t33784;
    (t33768, t33771, t33772, t33773, t33776, t33779, t33782, t33784, t33787)
}
