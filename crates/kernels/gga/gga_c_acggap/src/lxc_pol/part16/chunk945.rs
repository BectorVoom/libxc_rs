//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 945/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk945<F: Float>(t880: F, t9062: F, t1960: F, t5368: F, t310: F, t8995: F, t29997: F, t7963: F, t9029: F, t524: F, t9033: F, t4241: F, t7942: F) -> (F, F, F, F, F, F) {
    let t33648 = t9062 * t880;
    let t33656 = t1960 * t5368;
    let t33662 = F::cast_from(0.13170898365871023197e1_f64) * t310 * t8995;
    let t33672 = F::cast_from(0.17347256376410398924e1_f64) * t7963 * t29997 * t9029;
    let t33673 = t9033 * t524;
    let t33681 = F::cast_from(0.34694512752820797848e1_f64) * t7942 * t33673 * t4241;
    (t33648, t33656, t33662, t33672, t33673, t33681)
}
