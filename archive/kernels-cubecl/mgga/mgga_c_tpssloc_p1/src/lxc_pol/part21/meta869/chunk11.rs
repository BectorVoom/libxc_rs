//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3193/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3193<F: Float>(t18363: F, t3577: F, t45124: F, t11697: F, t18359: F, t15459: F, t15463: F, t15478: F, t15631: F, t15740: F, t18321: F, t18368: F, t3562: F, t45044: F, t45049: F, t45162: F, t53135: F, t53142: F, t53155: F, t53158: F, t53161: F, t53185: F, t53472: F) -> F {
    let t66334 = t3577 * t45124 * t18363;
    let t66337 = t3577 * t11697 * t18359;
    let t66353 = -t15740 * t15478 / F::cast_from(1152.0_f64) - t15740 * t15459 / F::cast_from(2304.0_f64) - t15740 * t15463 / F::cast_from(1152.0_f64) + F::cast_from(5.0_f64) / F::cast_from(10368.0_f64) * t66334 - t66337 / F::cast_from(1728.0_f64) - F::cast_from(5.0_f64) / F::cast_from(1944.0_f64) * t45044 + t53135 / F::cast_from(1728.0_f64) - F::cast_from(5.0_f64) / F::cast_from(62208.0_f64) * t45049 - t53472 * t15631 / F::cast_from(256.0_f64) + F::cast_from(11.0_f64) / F::cast_from(243.0_f64) * t18321 * t3562 - t53142 / F::cast_from(432.0_f64) - t45162 * t18368 / F::cast_from(1152.0_f64) - t53155 / F::cast_from(3456.0_f64) - t53158 / F::cast_from(1728.0_f64) + t53161 / F::cast_from(5184.0_f64) + t53185 / F::cast_from(2304.0_f64);
    t66353
}
