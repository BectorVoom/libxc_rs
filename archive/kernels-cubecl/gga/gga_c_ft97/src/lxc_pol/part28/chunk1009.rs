//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1009/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1009<F: Float>(t32545: F, t3255: F, t26113: F, t5710: F, t1286: F, t34365: F, t376: F, t1586: F, t34482: F, t136121: F, t137262: F, t137525: F, t25605: F, t28: F, t2976: F, t3109: F, t32380: F, t34577: F, t34585: F, t5495: F, t5501: F, t5508: F, t5618: F, t6414: F, t6562: F, t7286: F) -> (F, F, F, F) {
    let t144648 = t32545 * t3255;
    let t144657 = t5710 * t26113;
    let t144664 = t1286 * t376 * t34365;
    let t144666 = t1586 * t34482;
    let t144676 = -F::cast_from(2.0_f64) * t144648 - t136121 / F::cast_from(27.0_f64) + t6414 * t32380 / F::cast_from(6.0_f64) + t137262 + t1286 * t28 * t5618 * t6562 / F::cast_from(3.0_f64) - F::cast_from(4.0_f64) * t144657 - t2976 * t7286 - t3109 * t7286 + t5495 * t34577 / F::cast_from(6.0_f64) - t144664 / F::cast_from(3.0_f64) - t1286 * t28 * t144666 * t5508 / F::cast_from(3.0_f64) + t5501 * t137525 * t25605 / F::cast_from(9.0_f64) + t5495 * t34585 / F::cast_from(3.0_f64);
    (t144648, t144657, t144666, t144676)
}
