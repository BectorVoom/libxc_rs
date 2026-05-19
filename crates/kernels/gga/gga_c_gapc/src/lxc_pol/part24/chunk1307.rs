//! GGA_C_GAPC lxc pol — lxc_pol part 24 (v4rho2sigma2_3) CSE chunk 1307/1327 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part24_v4rho2sigma2_3_chunk1307<F: Float>(t34235: F, t34238: F, t34241: F, t34245: F, t34249: F, t34252: F, t34255: F, t34258: F, t34264: F, t34269: F, t34274: F, t10786: F, t1112: F, t1616: F) -> (F, F) {
    let t38051 = F::cast_from(0.20596571349374880758e-5_f64) * t34235 + F::cast_from(0.80043425406508130348e-8_f64) * t34238 + F::cast_from(0.69504740211613770836e-3_f64) * t34241 + F::cast_from(0.98326426188151041676e-8_f64) * t34245 - F::cast_from(0.32775475396050347226e-8_f64) * t34249 - F::cast_from(0.20596571349374880758e-4_f64) * t34252 - F::cast_from(0.65550950792100694451e-8_f64) * t34255 + F::cast_from(0.44197102999375800016e-7_f64) * t34258 + F::cast_from(0.2651826179962548001e-6_f64) * t34264 - F::cast_from(0.21914396903857167508e-6_f64) * t34269 + F::cast_from(0.15716489826578034486e-7_f64) * t34274;
    let t38060 = F::new(4.0) * t1616 * t1112 * t10786;
    (t38051, t38060)
}
