//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2221/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2221(t14501: f64, t23419: f64, t1015: f64, t23472: f64, t25678: f64, t14198: f64, t23544: f64, t4590: f64, t4596: f64, t4600: f64, t6717: f64, t82848: f64, t82956: f64, t83139: f64, t83153: f64, t83157: f64, t83159: f64, t83165: f64, t83167: f64, t83172: f64, t83206: f64) -> f64 {
    let t88704 = t23419 * t14501 / 1728.0_f64;
    let t88723 = 0.20186378047070195428e-3_f64 * t23472 * t1015 * t25678;
    let t88724 = t88704 - 0.20186378047070195428e-3_f64 * t83139 + t6717 * t14198 / 288.0_f64 - t83153 / 162.0_f64 - t83157 / 648.0_f64 - t83159 / 432.0_f64 + t83165 / 864.0_f64 + t83167 / 648.0_f64 + 19.0_f64 / 1296.0_f64 * t83172 + 0.10093189023535097714e-3_f64 * t83206 + 5.0_f64 / 3456.0_f64 * t23544 * t4590 - t82956 * t4596 / 72.0_f64 + t82848 * t4600 / 144.0_f64 + t88723;
    t88724
}
