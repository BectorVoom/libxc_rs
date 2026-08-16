//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 1984/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1984<F: Float>(t218: F, t816: F, t92993: F, t10685: F, t1946: F, t10671: F, t7033: F, t25255: F, t2689: F, t10690: F, t1945: F, t9646: F) -> (F, F, F, F, F) {
    let t92995 = t92993 * t218 * t816;
    let t92996 = F::cast_from(455.0_f64) / F::cast_from(1296.0_f64) * t92995;
    let t92997 = t1946 * t10685;
    let t92998 = F::cast_from(0.7558530601555998074e-1_f64) * t92997;
    let t92999 = t7033 * t10671;
    let t93000 = F::cast_from(0.25692334753583138159e-2_f64) * t92999;
    let t93001 = t2689 * t25255;
    let t93007 = t9646 * t1945 * t10690;
    (t92996, t92998, t93000, t93001, t93007)
}
