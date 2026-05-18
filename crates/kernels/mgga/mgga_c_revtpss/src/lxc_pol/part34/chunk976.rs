//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 976/1341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk976<F: Float>(t10698: F, t23114: F, t828: F, t23148: F, t855: F, t10824: F, t10826: F, t10885: F, t18459: F, t18475: F, t18485: F, t18487: F, t18491: F, t18518: F, t18532: F, t18623: F, t18644: F, t851: F) -> (F, F, F) {
    let t23342 = t10698 * t828 * t23114;
    let t23346 = t855 * t828 * t23148;
    let t23357 = F::new(0.30011812682648815881e-2) * t18459 - F::new(0.25724410870841842183e-1) * t851 * t23342 - F::new(0.85748036236139473944e-3) * t851 * t23346 - F::new(0.60023625365297631762e-1) * t18475 + F::new(0.12004725073059526352e-1) * t18485 - t10824 + t10826 - F::new(0.60023625365297631762e-2) * t18487 + F::new(0.30011812682648815881e-2) * t18491 - t10885 - F::new(0.24009450146119052704e-1) * t18518 - F::new(0.38115002106963996168e-4) * t18532 - F::new(0.38115002106963996168e-4) * t18623 + F::new(0.30492001685571196935e-3) * t18644;
    (t23342, t23346, t23357)
}
