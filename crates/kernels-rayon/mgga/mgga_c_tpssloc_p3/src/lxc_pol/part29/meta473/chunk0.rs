//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 1809/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1809(t1268: f64, t22479: f64, t12461: f64, t3698: f64, t2019: f64, t1983: f64, t12521: f64, t1873: f64, t12524: f64, t7015: f64, t3938: f64, t6534: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t23854 = 2.0_f64 * t1268 * t22479;
    let t23857 = t12461 * t3698;
    let t23858 = t2019 * t23857;
    let t23860 = 2.0_f64 * t1983 * t23858;
    let t23886 = 0.135e2_f64 * t12521 * t1873;
    let t23888 = 54.0_f64 * t12524 * t7015;
    let t23890 = 27.0_f64 * t3938 * t6534;
    (t23854, t23857, t23858, t23860, t23886, t23888, t23890)
}
