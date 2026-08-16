//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1276/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1276(t27561: f64, t7327: f64, t1209: f64, t85964: f64, t3032: f64, t475: f64, t210: f64, t24810: f64, t24848: f64, t24594: f64, t24847: f64, t974: f64) -> (f64, f64, f64, f64, f64) {
    let t86015 = t7327 * t27561;
    let t86022 = t85964 * t1209;
    let t86023 = t3032 * t475;
    let t86036 = t24810 * t210;
    let t86037 = t86036 * t24848;
    let t86076 = t24847 * t974 * t24594;
    (t86015, t86022, t86023, t86037, t86076)
}
