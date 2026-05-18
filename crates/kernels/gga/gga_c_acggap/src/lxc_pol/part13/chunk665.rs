//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 665/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk665<F: Float>(t181: F, t315: F, t1219: F, t556: F, t871: F, t119: F, t3932: F, t3935: F, t3939: F, t5359: F, t5361: F, t5364: F, t5365: F, t5369: F, t5372: F, t5375: F, t5381: F, t5382: F) -> (F, F) {
    let t5384 = t315 * t181;
    let t5385 = t1219 * t556;
    let t5386 = t5385 * t871;
    let t5388 = F::new(0.26341796731742046394e1) * t5384 * t5386;
    let t5390 = -t5359 - F::new(0.13170898365871023197e1) * t5361 + t5364 - F::new(0.65854491829355115987e0) * t5365 + F::new(0.65854491829355115987e0) * t5369 + F::new(0.13170898365871023197e1) * t5372 + F::new(0.65854491829355115987e0) * t119 * t5375 - t3932 + t5381 - F::new(0.65854491829355115987e0) * t5382 - t5388 - F::new(0.13170898365871023197e1) * t3935 + t3939;
    (t5386, t5390)
}
