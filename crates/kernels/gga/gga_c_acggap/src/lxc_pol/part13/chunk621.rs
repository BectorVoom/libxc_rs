//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 621/1066 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk621<F: Float>(t1658: F, t322: F, t449: F, t316: F, t3892: F, t557: F, t181: F, t315: F, t1219: F, t556: F, t871: F, t119: F, t3932: F, t3935: F, t3939: F, t5359: F, t5361: F, t5364: F, t5365: F, t5369: F, t5372: F, t5375: F) -> (F, F, F) {
    let t5378 = t1658 * t322;
    let t5379 = t449 * t5378;
    let t5381 = 0.13170898365871023197e1 * t316 * t5379;
    let t5382 = t3892 * t557;
    let t5384 = t315 * t181;
    let t5385 = t1219 * t556;
    let t5386 = t5385 * t871;
    let t5388 = 0.26341796731742046394e1 * t5384 * t5386;
    let t5390 = -t5359 - 0.13170898365871023197e1 * t5361 + t5364 - 0.65854491829355115987e0 * t5365 + 0.65854491829355115987e0 * t5369 + 0.13170898365871023197e1 * t5372 + 0.65854491829355115987e0 * t119 * t5375 - t3932 + t5381 - 0.65854491829355115987e0 * t5382 - t5388 - 0.13170898365871023197e1 * t3935 + t3939;
    (t5379, t5386, t5390)
}
