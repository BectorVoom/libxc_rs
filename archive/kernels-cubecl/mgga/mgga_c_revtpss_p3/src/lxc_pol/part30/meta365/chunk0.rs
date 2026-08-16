//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 1391/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1391<F: Float>(t12831: F, t3624: F, t221: F, t462: F, t68: F, t461: F, t1209: F, t3766: F, t5330: F, t11772: F, t3623: F, t3717: F) -> (F, F, F, F, F, F, F) {
    let t12832 = t12831 * t3624;
    let t12851 = t221 * t68 * t462;
    let t12853 = F::cast_from(5.0_f64) / F::cast_from(1296.0_f64) * t461 * t12851;
    let t12854 = t1209 * t3766;
    let t12855 = t12854 * t5330;
    let t12865 = t3623 * t11772;
    let t12866 = t3717 * t12865;
    (t12832, t12851, t12853, t12854, t12855, t12865, t12866)
}
