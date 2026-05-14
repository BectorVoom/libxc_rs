//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 707/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk707<F: Float>(t33821: F, t33822: F, t684: F, t33820: F, t294: F, t7639: F, t7242: F, t7584: F, t824: F, t7512: F, t7638: F, t1476: F, t6260: F, t7641: F, t2781: F, t33812: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t33824 = t33821 * t33822 * t684;
    let t33825 = t33820 * t33824;
    let t33828 = 1.0 / t7639 / t294;
    let t33829 = t33828 * t7242;
    let t33830 = t7584 * t824;
    let t33831 = t33829 * t33830;
    let t33833 = t7638 * t7512 * t33831;
    let t33835 = t1476 * t6260;
    let t33836 = t7641 * t33835;
    let t33838 = t7638 * t7512 * t33836;
    let t33840 = t2781 * t33812;
    (t33824, t33825, t33828, t33829, t33830, t33831, t33833, t33835, t33836, t33838, t33840)
}
