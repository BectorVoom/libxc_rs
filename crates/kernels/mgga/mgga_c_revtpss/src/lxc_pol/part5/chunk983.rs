//! MGGA_C_REVTPSS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 983/1422 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part5_v3rho3_2_chunk983<F: Float>(t2453: F, t4100: F, t1398: F, t281: F, t543: F, t68: F, t10115: F, t562: F, t2435: F, t3903: F, t1445: F, t3895: F) -> (F, F, F, F, F) {
    let t10139 = t2453 * t4100;
    let t10142 = t281 * t68 * t1398 * t543;
    let t10143 = t10139 * t10142;
    let t10157 = F::cast_from(0.11044544084478153697e-3_f64) * t10115 * t562;
    let t10160 = t2435 * t3903;
    let t10162 = t3895 * t1445;
    (t10139, t10143, t10157, t10160, t10162)
}
