//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 834/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk834(t213: f64, t683: f64, t171: f64, t6793: f64, t1113: f64, t1420: f64, t17807: f64, t27521: f64, t30671: f64, t30779: f64, t33388: f64, t33394: f64, t33434: f64, t33436: f64, t33445: f64, t35395: f64, t35402: f64, t35406: f64, t35410: f64, t35416: f64, t35420: f64, t35426: f64, t35427: f64, t35431: f64, t35435: f64, t52: f64, t6758: f64, t7456: f64, t7457: f64, t7470: f64) -> (f64, f64, f64) {
    let t35437 = t683 * t213;
    let t35438 = t6793 * t171 * t35437;
    let t35441 = -0.44455354858818847408e-2_f64 * t7456 * t52 * t7457 * t1113 + 0.22227677429409423704e-2_f64 * t30671 * t35395 - 0.22227677429409423704e-2_f64 * t33388 * t35395 + 0.52700762016626893448e-4_f64 * t7456 * t35402 + 0.39129660776942540761e-2_f64 * t33445 * t35406 + 0.68116566383613497688e-3_f64 * t30779 * t7470 * t35410 - 0.68116566383613497688e-3_f64 * t27521 * t35416 - 0.76612330055555555556e-1_f64 * t35420 * t1420 - 0.22979081259345929704e-6_f64 * t17807 * t33394 * t6758 + 0.11738898233082762228e-1_f64 * t35426 * t33436 * t35427 - 0.17608347349624143343e-1_f64 * t33434 * t33436 * t35431 + 0.42300125954037691564e-4_f64 * t35435 * t35438;
    (t35437, t35438, t35441)
}
