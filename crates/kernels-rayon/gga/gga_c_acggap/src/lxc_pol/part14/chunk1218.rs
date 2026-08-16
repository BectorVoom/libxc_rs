//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 1218/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk1218(t1815: f64, t309: f64, t7963: f64, t9033: f64, t38778: f64, t7942: f64, t463: f64, t32150: f64, t32157: f64, t32161: f64, t32163: f64, t32167: f64, t32168: f64, t32176: f64, t32180: f64, t32183: f64, t36460: f64, t36473: f64, t7931: f64, t8437: f64, t9003: f64) -> f64 {
    let t40861 = t1815 * t309;
    let t40863 = t7963 * t9033 * t40861;
    let t40866 = t7942 * t9033 * t38778;
    let t40868 = t1815 * t463;
    let t40880 = -0.17347256376410398924e1_f64 * t40863 + 0.17347256376410398924e1_f64 * t40866 + 0.17347256376410398924e1_f64 * t7931 * t9033 * t40868 + t32150 - 0.17347256376410398924e1_f64 * t36460 + 0.8673628188205199462e0_f64 * t32157 - 0.8673628188205199462e0_f64 * t32161 + 0.8673628188205199462e0_f64 * t32163 + 0.17347256376410398924e1_f64 * t9003 * t8437 - t32167 - 0.8673628188205199462e0_f64 * t32168 - t32176 + t32180 - t36473 - 0.34694512752820797848e1_f64 * t32183;
    t40880
}
