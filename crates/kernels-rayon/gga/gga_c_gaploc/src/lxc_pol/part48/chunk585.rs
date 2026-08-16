//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 585/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk585(t10648: f64, t10711: f64, t10748: f64, t10793: f64, t3455: f64, t747: f64, t3459: f64, t841: f64, t1052: f64, t2728: f64, t1022: f64, t830: f64) -> (f64, f64, f64, f64, f64) {
    let t10795 = t10648 + t10711 + t10748 + t10793;
    let t10800 = t3455 * t747;
    let t10802 = t3459 * t841;
    let t10805 = t1052 * t2728;
    let t10809 = t830 * t1022;
    (t10795, t10800, t10802, t10805, t10809)
}
