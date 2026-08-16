//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2002/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2002<F: Float>(t2710: F, t826: F, t92986: F, t10631: F, t10886: F, t7028: F, t159: F, t8779: F, t218: F, t816: F, t10685: F, t1946: F) -> (F, F, F, F, F) {
    let t92988 = t2710 * t92986 * t826;
    let t92989 = F::cast_from(0.16264433699083676445e-3_f64) * t92988;
    let t92991 = t10886 * t7028 * t10631;
    let t92993 = t8779 * t159;
    let t92995 = t92993 * t218 * t816;
    let t92996 = F::cast_from(455.0_f64) / F::cast_from(1296.0_f64) * t92995;
    let t92997 = t1946 * t10685;
    (t92989, t92991, t92993, t92996, t92997)
}
