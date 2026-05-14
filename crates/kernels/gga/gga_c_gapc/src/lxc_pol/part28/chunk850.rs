//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 850/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk850<F: Float>(t10247: F, t10250: F, t10253: F, t10258: F, t10262: F, t10267: F, t10271: F, t10274: F, t10276: F, t10278: F, t10281: F, t10284: F, t10290: F, t10295: F, t10299: F, t10303: F, t10306: F, t10311: F, t10314: F, t10317: F, t10319: F, t10321: F) -> (F, F) {
    let t11111 = 0.13919347044349879094e-6 * t10247 - 0.41758041133049637282e-5 * t10250 - 0.23485962392041415794e-5 * t10253 - 0.14984533297005590806e-5 * t10258 - 0.24748599044854085031e-6 * t10262 + 0.41758041133049637282e-5 * t10267 - 0.23485962392041415794e-4 * t10271 - 0.66812865812879419652e-4 * t10274 + 0.1487444284829289667e-3 * t10276 + 0.46808827823026988424e-4 * t10278 - 0.23485962392041415794e-5 * t10281;
    let t11123 = -0.41758041133049637282e-5 * t10284 + 0.11636624900248636096e-6 * t10290 + 0.685007236434541294e-5 * t10295 + 0.41758041133049637282e-5 * t10299 + 0.22833574547818043134e-6 * t10303 + 0.3757753982726626527e-4 * t10306 + 0.66812865812879419652e-4 * t10311 + 0.23485962392041415794e-5 * t10314 + 0.16414765573575218917e-4 * t10317 + 0.66812865812879419652e-4 * t10319 - 0.15589668689671864586e-3 * t10321;
    (t11111, t11123)
}
