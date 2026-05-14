//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1043/1092 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1043<F: Float>(t35348: F, t35359: F, t31108: F, t31111: F, t31118: F, t31120: F, t31124: F, t31126: F, t31128: F, t31131: F, t31140: F, t31143: F, t31160: F, t31162: F, t35342: F, t35350: F, t35357: F, t35366: F) -> (F,) {
    let t37498 = 0.14291339372689912324e-2 * t35348;
    let t37504 = 0.39221875e0 * t35359;
    let t37510 = 0.61125e-1 * t31108 - 7.0 / 24.0 * t31111 - 0.42874018118069736972e-2 * t35342 + 0.31448092289604152068e-2 * t31118 - 0.37737710747524982483e-2 * t31120 - 0.6289618457920830414e-2 * t31124 - t37498 - 0.85748036236139473944e-3 * t35350 + 0.264875e0 * t31126 - 0.11433071498151929859e-2 * t31128 - t31131 / 32.0 + 0.1528125e-1 * t35357 + t37504 + 0.305625e-1 * t31140 - 7.0 / 36.0 * t31143 - 0.68598428988911579156e-2 * t31160 - t35366 / 2.0 + 0.25724410870841842184e-2 * t31162;
    (t37510,)
}
