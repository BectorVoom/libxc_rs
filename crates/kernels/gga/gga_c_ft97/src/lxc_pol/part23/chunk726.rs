//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 726/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk726<F: Float>(t13839: F, t3870: F, t5147: F, t761: F, t684: F, t2606: F, t5134: F, t681: F, t89: F, t1168: F, t3972: F, t2568: F, t242: F, t10134: F, t14240: F, t14281: F, t14283: F, t18709: F, t18714: F, t18718: F, t18721: F, t18726: F, t18731: F, t18734: F, t1901: F, t446: F) -> (F, F, F) {
    let t18737 = t13839 * t3870;
    let t18740 = t761 * t5147;
    let t18741 = t18740 * t684;
    let t18742 = t2606 * t18741;
    let t18746 = t89 * t681 * t5134;
    let t18749 = t1168 * t3972;
    let t18750 = t2568 * t18749;
    let t18751 = t242 * t18750;
    let t18754 = t1901 * t18709 / 9.0 + 2.0 / 9.0 * t1901 * t18714 - 2.0 / 27.0 * t1901 * t18718 - t14240 + 2.0 / 9.0 * t1901 * t18721 + 2.0 / 9.0 * t1901 * t18726 + t1901 * t18731 / 9.0 + 2.0 / 9.0 * t1901 * t18734 + 2.0 / 9.0 * t1901 * t18737 + t1901 * t18742 / 9.0 - t18746 / 9.0 + t14281 + t14283 - 4.0 / 81.0 * t10134 + 4.0 / 3.0 * t446 * t18751;
    (t18741, t18749, t18754)
}
