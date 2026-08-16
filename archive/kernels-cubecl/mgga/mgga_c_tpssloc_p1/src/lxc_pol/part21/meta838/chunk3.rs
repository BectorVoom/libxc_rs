//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2993/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2993<F: Float>(t10231: F, t17157: F, t973: F, t17161: F, t17183: F, t2970: F, t17178: F, t17599: F, t17602: F, t17994: F, t2960: F, t43228: F, t50242: F, t50250: F, t50255: F, t50258: F, t50262: F, t59730: F, t59746: F, t977: F) -> F {
    let t62657 = t973 * t10231 * t17157;
    let t62660 = t973 * t10231 * t17161;
    let t62663 = t973 * t2970 * t17183;
    let t62666 = t973 * t10231 * t17178;
    let t62680 = F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t2960 * t17994 - F::cast_from(4.0_f64) / F::cast_from(81.0_f64) * t2960 * t17599 - F::cast_from(14.0_f64) / F::cast_from(243.0_f64) * t2960 * t17602 - t62657 / F::cast_from(54.0_f64) + t62660 / F::cast_from(162.0_f64) - t62663 / F::cast_from(216.0_f64) + t62666 / F::cast_from(324.0_f64) - t973 * t977 * t59730 / F::cast_from(72.0_f64) + t43228 / F::cast_from(1296.0_f64) - t50242 / F::cast_from(108.0_f64) - t50250 / F::cast_from(216.0_f64) + t50255 / F::cast_from(384.0_f64) + t50258 / F::cast_from(3456.0_f64) - t50262 / F::cast_from(5184.0_f64) + t973 * t977 * t59746 / F::cast_from(48.0_f64);
    t62680
}
