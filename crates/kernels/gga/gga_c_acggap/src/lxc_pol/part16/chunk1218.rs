//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 1218/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk1218<F: Float>(t1815: F, t309: F, t7963: F, t9033: F, t38778: F, t7942: F, t463: F, t32150: F, t32157: F, t32161: F, t32163: F, t32167: F, t32168: F, t32176: F, t32180: F, t32183: F, t36460: F, t36473: F, t7931: F, t8437: F, t9003: F) -> F {
    let t40861 = t1815 * t309;
    let t40863 = t7963 * t9033 * t40861;
    let t40866 = t7942 * t9033 * t38778;
    let t40868 = t1815 * t463;
    let t40880 = -F::cast_from(0.17347256376410398924e1_f64) * t40863 + F::cast_from(0.17347256376410398924e1_f64) * t40866 + F::cast_from(0.17347256376410398924e1_f64) * t7931 * t9033 * t40868 + t32150 - F::cast_from(0.17347256376410398924e1_f64) * t36460 + F::cast_from(0.8673628188205199462e0_f64) * t32157 - F::cast_from(0.8673628188205199462e0_f64) * t32161 + F::cast_from(0.8673628188205199462e0_f64) * t32163 + F::cast_from(0.17347256376410398924e1_f64) * t9003 * t8437 - t32167 - F::cast_from(0.8673628188205199462e0_f64) * t32168 - t32176 + t32180 - t36473 - F::cast_from(0.34694512752820797848e1_f64) * t32183;
    t40880
}
