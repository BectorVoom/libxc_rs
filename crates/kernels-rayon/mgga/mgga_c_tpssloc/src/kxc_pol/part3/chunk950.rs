//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 3 (v3rho3_1) CSE chunk 950/1255 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_kxc_pol_part3_v3rho3_1_chunk950(t11698: f64, t3577: f64, t248: f64, t3494: f64, t3570: f64, t1213: f64, t3490: f64, t3523: f64, t1190: f64, t3030: f64, t3032: f64, t3505: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t11699 = t3577 * t11698;
    let t11702 = t248 * t3570 * t3494;
    let t11703 = t1213 * t11702;
    let t11705 = t3490 * t3523;
    let t11707 = t1190 * t3030;
    let t11708 = t11707 * t3032;
    let t11709 = t11708 * t3505;
    (t11699, t11703, t11705, t11707, t11708, t11709)
}
