//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1435/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1435(t1128: f64, t11455: f64, t3324: f64, t3356: f64, t43748: f64, t43750: f64, t43780: f64, t43782: f64, t43784: f64, t43786: f64, t43788: f64, t43794: f64, t43798: f64, t43802: f64, t43806: f64) -> (f64, f64, f64) {
    let t44295 = t11455 * t1128;
    let t44300 = t3324 * t3356;
    let t44314 = -0.3044148148148148148e-1_f64 * t43748 - 0.25367901234567901233e-1_f64 * t43750 + 0.45662222222222222221e-1_f64 * t43780 + 0.9132444444444444444e-1_f64 * t43782 + 0.9132444444444444444e-1_f64 * t43784 - 0.13698666666666666667e0_f64 * t43786 - 0.22831111111111111111e-1_f64 * t43788 + 0.2283111111111111111e0_f64 * t43794 - 0.41095999999999999999e0_f64 * t43798 + 0.41096e0_f64 * t43802 + 0.17123333333333333333e-1_f64 * t43806;
    (t44295, t44300, t44314)
}
