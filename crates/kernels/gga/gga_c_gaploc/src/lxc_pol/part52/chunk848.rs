//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 848/1013 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk848<F: Float>(t2508: F, t2580: F, t45087: F, t11603: F, t1897: F, t7068: F, t7226: F, t3650: F, t7671: F, t2936: F, t33561: F, t13548: F, t2549: F) -> (F, F, F, F, F) {
    let t45090 = F::cast_from(0.15381052460284448567e-1_f64) * t2508 * t2580 * t45087;
    let t45101 = F::cast_from(0.46143157380853345701e-1_f64) * t1897 * t7226 * t11603 * t7068;
    let t45104 = F::cast_from(0.53833683610995569986e-1_f64) * t1897 * t3650 * t7671;
    let t45107 = F::cast_from(0.10766736722199113997e0_f64) * t2508 * t2936 * t33561;
    let t45108 = t2549 * t13548;
    (t45090, t45101, t45104, t45107, t45108)
}
