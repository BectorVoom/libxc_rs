//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2130/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2130(t776: f64, t865: f64, t22986: f64, t23270: f64, t25044: f64, t82147: f64, t13377: f64, t1880: f64, t214: f64, t225: f64, t258: f64, t1887: f64, t81956: f64) -> (f64, f64, f64, f64, f64) {
    let t87036 = t776 * t865;
    let t87039 = t22986 * t23270 * t25044 * t87036;
    let t87042 = 0.52089578783527170489e-1_f64 * t82147;
    let t87047 = t1880 * t214 * t13377 * t225 * t258;
    let t87049 = t81956 * t1887;
    (t87036, t87039, t87042, t87047, t87049)
}
