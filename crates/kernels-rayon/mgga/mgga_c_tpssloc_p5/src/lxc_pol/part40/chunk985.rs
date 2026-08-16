//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 40 (v4rho3tau_4) CSE chunk 985/1303 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part40_v4rho3tau_4_chunk985(t2970: f64, t4522: f64, t973: f64, t10254: f64, t3961: f64, t10236: f64, t10189: f64, t1597: f64, t2990: f64, t2986: f64, t2987: f64, t4540: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t13828 = t2970 * t4522;
    let t13830 = 0.18518518518518518518e-3_f64 * t973 * t13828;
    let t13835 = t10254 * t3961;
    let t13839 = t10236 * t3961;
    let t13847 = t10189 * t1597;
    let t13848 = t13847 * t2990;
    let t13850 = 0.18518518518518518518e-3_f64 * t2986 * t13848;
    let t13851 = t2987 * t4540;
    (t13830, t13835, t13839, t13847, t13850, t13851)
}
