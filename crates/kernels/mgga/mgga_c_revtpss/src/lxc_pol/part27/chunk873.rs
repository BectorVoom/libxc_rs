//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 873/1170 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk873<F: Float>(t11571: F, t324: F, t11132: F, t11134: F, t11136: F, t11138: F, t11140: F, t11147: F, t11153: F, t11158: F, t11162: F, t11167: F, t11171: F, t11291: F, t11293: F, t11296: F, t11303: F, t11382: F, t11390: F, t11521: F, t11525: F, t11530: F, t11533: F, t11547: F, t11548: F, t11551: F, t11554: F, t11557: F, t2945: F, t2968: F, t2987: F, t2989: F, t3012: F, t311: F) -> (F, F) {
    let t11572 = t11571 * t324;
    let t11574 = 0.53272592592592592592e-1 * t11132;
    let t11585 = -t11574 - 0.2283111111111111111e-1 * t11134 + 0.11415555555555555555e-1 * t11136 - 0.34246666666666666665e-1 * t11138 + 0.17123333333333333333e-1 * t11140 - 0.19025925925925925925e-1 * t11147 + 0.68493333333333333331e-1 * t11153 - 0.34246666666666666665e-1 * t11158 - 0.10274e0 * t11162 + 0.10274e0 * t11167 - 0.17123333333333333333e-1 * t11171;
    let t11588 = -0.35089341735807877242e1 * t2987 * t11521 + 0.51947577317044391277e2 * t3012 * t11525 + t11530 - t11533 + t11547 - t11291 - t11293 - t11296 + t11303 - t11382 - t11390 - 6.0 * t11548 * t2945 + 6.0 * t2968 * t11551 - 0.35089341735807877242e1 * t11554 * t2989 + 0.35089341735807877242e1 * t3012 * t11557 - 0.19751673498613801407e-1 * t11572 - 0.310907e-1 * t11585 * t311;
    (t11572, t11588)
}
