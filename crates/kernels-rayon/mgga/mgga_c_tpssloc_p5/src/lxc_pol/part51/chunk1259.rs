//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1259/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1259(t225: f64, t26708: f64, t1509: f64, t7084: f64, t2047: f64, t4233: f64, t26732: f64, t26734: f64, t10109: f64, t7106: f64, t10143: f64, t7844: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t92439 = t26708 * t225;
    let t92552 = t7084 * t1509;
    let t92745 = t2047 * t4233;
    let t92847 = t26732 * t225;
    let t92939 = t26734 * t225;
    let t92981 = t10109 * t7106;
    let t93000 = t7844 * t10143;
    (t92439, t92552, t92745, t92847, t92939, t92981, t93000)
}
