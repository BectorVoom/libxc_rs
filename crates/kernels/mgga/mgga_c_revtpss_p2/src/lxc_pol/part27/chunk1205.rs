//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1205/1333 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1205<F: Float>(t92988: F, t10631: F, t10886: F, t7028: F, t159: F, t8779: F, t218: F, t816: F, t10685: F, t1946: F, t10671: F, t7033: F) -> (F, F, F, F, F, F) {
    let t92989 = F::cast_from(0.16264433699083676445e-3_f64) * t92988;
    let t92991 = t10886 * t7028 * t10631;
    let t92993 = t8779 * t159;
    let t92995 = t92993 * t218 * t816;
    let t92996 = F::new(455.0) / F::new(1296.0) * t92995;
    let t92997 = t1946 * t10685;
    let t92998 = F::cast_from(0.7558530601555998074e-1_f64) * t92997;
    let t92999 = t7033 * t10671;
    (t92989, t92991, t92993, t92996, t92998, t92999)
}
