//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 628/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk628<F: Float>(t13839: F, t2609: F, t13827: F, t241: F, t258: F, t2409: F, t3897: F, t2599: F, t2373: F, t992: F, t2600: F, t10079: F, t2567: F, t668: F, t2569: F, t2606: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t13840 = t13839 * t2609;
    let t13844 = t241 * t13827 * t258;
    let t13848 = t3897 * t2409;
    let t13849 = t2599 * t13848;
    let t13852 = t992 * t2373;
    let t13853 = t2600 * t13852;
    let t13854 = t10079 * t13853;
    let t13857 = t2567 * t668;
    let t13858 = t992 * t2569;
    let t13859 = t13857 * t13858;
    let t13860 = t2606 * t13859;
    (t13840, t13844, t13848, t13849, t13852, t13853, t13854, t13858, t13859, t13860)
}
