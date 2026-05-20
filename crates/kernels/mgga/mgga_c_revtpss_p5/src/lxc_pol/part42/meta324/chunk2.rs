//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 1110/1505 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1110<F: Float>(t10022: F, t14230: F, t2782: F, t1892: F, t4086: F, t786: F, t4104: F, t2470: F, t5740: F, t4101: F, t1432: F, t5763: F) -> (F, F, F, F, F) {
    let t14231 = t10022 * t14230;
    let t14233 = F::cast_from(0.21951497276451705328e-1_f64) * t2782 * t14231;
    let t14238 = t4086 * t1892;
    let t14239 = t786 * t14238;
    let t14241 = F::cast_from(0.19514881078765566038e-1_f64) * t14239 * t4104;
    let t14242 = t5740 * t2470;
    let t14243 = t4101 * t14242;
    let t14252 = t1432 * t5763 * t2470;
    (t14233, t14239, t14241, t14243, t14252)
}
