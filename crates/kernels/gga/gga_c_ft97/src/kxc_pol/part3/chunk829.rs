//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 829/887 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk829<F: Float>(t18750: F, t242: F, t10134: F, t14240: F, t14281: F, t14283: F, t18709: F, t18714: F, t18718: F, t18721: F, t18726: F, t18731: F, t18734: F, t18737: F, t18742: F, t18746: F, t1901: F, t446: F) -> (F,) {
    let t18751 = t242 * t18750;
    let t18754 = t1901 * t18709 / 9.0 + 2.0 / 9.0 * t1901 * t18714 - 2.0 / 27.0 * t1901 * t18718 - t14240 + 2.0 / 9.0 * t1901 * t18721 + 2.0 / 9.0 * t1901 * t18726 + t1901 * t18731 / 9.0 + 2.0 / 9.0 * t1901 * t18734 + 2.0 / 9.0 * t1901 * t18737 + t1901 * t18742 / 9.0 - t18746 / 9.0 + t14281 + t14283 - 4.0 / 81.0 * t10134 + 4.0 / 3.0 * t446 * t18751;
    (t18754,)
}
