//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 950/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk950<F: Float>(t1125: F, t9672: F, t242: F, t2846: F, t3090: F, t2845: F, t400: F, t2192: F, t359: F, t461: F, t460: F, t3097: F, t774: F) -> (F, F, F, F, F) {
    let t9673 = t1125 * t9672;
    let t9676 = t242 * t3090 * t2846;
    let t9677 = t1125 * t9676;
    let t9684 = F::cast_from(1.0_f64) / t400 / t2845;
    let t9699 = t359 * t2192 * t461;
    let t9701 = t460 * t9699 / F::cast_from(10368.0_f64);
    let t9702 = t774 * t3097;
    (t9673, t9677, t9684, t9701, t9702)
}
