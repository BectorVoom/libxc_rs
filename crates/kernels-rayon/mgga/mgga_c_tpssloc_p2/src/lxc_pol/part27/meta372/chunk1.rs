//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 1531/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1531(t13823: f64, t973: f64, t2970: f64, t4522: f64, t6733: f64, t884: f64, t4531: f64, t10254: f64, t3961: f64, t2988: f64, t10236: f64, t10235: f64) -> (f64, f64, f64, f64, f64) {
    let t13825 = 0.55555555555555555554e-3_f64 * t973 * t13823;
    let t13828 = t2970 * t4522;
    let t13830 = 0.18518518518518518518e-3_f64 * t973 * t13828;
    let t13831 = t6733 * t884;
    let t13832 = t4531 * t13831;
    let t13835 = t10254 * t3961;
    let t13836 = t2988 * t13835;
    let t13839 = t10236 * t3961;
    let t13840 = t10235 * t13839;
    (t13825, t13830, t13832, t13836, t13840)
}
