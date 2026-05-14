//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 787/869 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk787<F: Float>(t2405: F, t3869: F, t3891: F, t3972: F, t761: F, t684: F, t2606: F, t3870: F, t9787: F, t11593: F, t13894: F, t13899: F, t13903: F, t13905: F, t13907: F, t13911: F, t13915: F, t13919: F, t13924: F, t13929: F, t13933: F, t13935: F, t1901: F, t446: F) -> (F,) {
    let t13938 = t3869 * t2405;
    let t13939 = t3891 * t13938;
    let t13942 = t761 * t3972;
    let t13943 = t13942 * t684;
    let t13944 = t2606 * t13943;
    let t13947 = t9787 * t3870;
    let t13950 = -4.0 / 9.0 * t11593 * t13894 - 4.0 / 9.0 * t11593 * t13899 + t13903 + t13905 + 2.0 / 3.0 * t446 * t13907 - 2.0 / 3.0 * t446 * t13911 - t446 * t13915 / 3.0 - t446 * t13919 / 3.0 + t446 * t13924 / 3.0 + 2.0 / 3.0 * t446 * t13929 + t13933 + t1901 * t13935 / 9.0 + 2.0 / 27.0 * t1901 * t13939 + 2.0 / 9.0 * t1901 * t13944 + 2.0 / 9.0 * t1901 * t13947;
    (t13950,)
}
