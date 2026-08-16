//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 3 (v3rho3_1) CSE chunk 1140/1255 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part3_v3rho3_1_chunk1140(t11211: f64, t11213: f64, t11369: f64, t11372: f64, t14702: f64, t14705: f64, t14708: f64, t14711: f64, t14713: f64, t14759: f64, t14776: f64, t14779: f64, t14782: f64, t14784: f64, t14787: f64, t14790: f64, t14793: f64, t14796: f64, t14799: f64, t14802: f64, t14805: f64, t14827: f64) -> f64 {
    let t14829 = -t11369 - t11372 + 0.13418888888888888889e0_f64 * t14702 - t14705 + 0.301925e0_f64 * t14708 - t14711 + 0.82785e-1_f64 * t14713 + 0.258925e1_f64 * t14759 + 0.18396666666666666667e0_f64 * t11211 + 0.18396666666666666667e-1_f64 * t11213 + t14776 + 0.36793333333333333333e-1_f64 * t14779 - t14782 - 0.5519e-1_f64 * t14784 - 0.27595e-1_f64 * t14787 - 0.16557e0_f64 * t14790 + 0.33114e0_f64 * t14793 + 0.16557e0_f64 * t14796 + 0.49671e0_f64 * t14799 + 0.19419375e1_f64 * t14802 - 0.412621875e-1_f64 * t14805 + t14827;
    t14829
}
