//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1096/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1096<F: Float>(t86: F, t113: F, t144337: F, t144372: F, t144411: F, t144442: F, t144569: F, t144613: F, t144647: F, t144676: F, t144703: F, t144731: F, t144763: F, t145733: F, t145769: F, t146987: F, t147004: F, t147040: F, t1577: F, t18: F, t32650: F, t34791: F, t5: F, t505: F, t7293: F, t992: F) -> F {
    let t87 = F::cast_from(10000000.0_f64) <= t86;
    let t147059 = piecewise3::<F>(t87, F::cast_from(0.0_f64), t5 * (t144337 + t144372 + t144411 + t144442 + t144569 + t144613 + t144647 + t144676 + t144703 + t144731 + t144763 + t145733 + t145769 + t146987 + t147004 + t147040) * t113 / F::cast_from(4.0_f64) + t5 * t34791 * t505 / F::cast_from(4.0_f64) + t5 * t32650 * t992 / F::cast_from(4.0_f64) - t5 * t7293 * t18 * t1577 / F::cast_from(2.0_f64));
    t147059
}
