//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2299/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2299<F: Float>(t8070: F, t85660: F, t225: F, t27654: F, t24574: F, t27484: F, t1244: F, t1246: F, t15018: F, t15426: F, t2152: F, t24589: F, t24776: F, t24812: F, t24820: F, t24821: F, t24833: F, t24849: F, t27460: F, t27510: F, t27532: F, t3243: F, t5011: F, t5075: F, t7283: F, t7327: F, t7348: F, t7364: F, t7373: F, t85883: F, t85918: F) -> F {
    let t95033 = t85660 * t8070;
    let t95035 = t27654 * t225;
    let t95048 = F::cast_from(0.54831135561607547884e-2_f64) * t24574 * t27484;
    let t95058 = -F::cast_from(0.82246703342411321825e-2_f64) * t24812 * t24820 * t15018 * t24821 - F::cast_from(0.27415567780803773942e-2_f64) * t85883 + F::cast_from(0.18277045187202515961e-2_f64) * t95033 + F::cast_from(0.54831135561607547884e-2_f64) * t24589 * t95035 * t7364 - F::cast_from(0.54831135561607547884e-2_f64) * t24849 * t7327 * t5075 * t27532 + F::cast_from(2.0_f64) * t1244 * t7348 * t5011 * t1246 - t95048 + F::cast_from(0.36554090374405031923e-2_f64) * t7283 * t24776 * t27460 * t3243 + t15426 * t2152 - F::cast_from(0.36554090374405031922e-2_f64) * t85918 - F::cast_from(0.16449340668482264365e-1_f64) * t7373 * t24833 * t27510;
    t95058
}
