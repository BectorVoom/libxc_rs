//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2339/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2339(t1858: f64, t7758: f64, t2029: f64, t6470: f64, t1851: f64, t7774: f64, t100867: f64, t1396: f64, t1852: f64, t26555: f64, t28904: f64, t3: f64, t5381: f64, t580: f64, t6483: f64, t7003: f64, t7759: f64, t86579: f64, t91813: f64, t91816: f64, t91818: f64, t91824: f64) -> f64 {
    let t100949 = t7758 * t1858;
    let t100952 = t6470 * t2029;
    let t100960 = t1851 * t7774;
    let t100962 = t100867 * t3 * t580 + t1396 * t28904 + 2.0_f64 * t1852 * t26555 + 2.0_f64 * t5381 * t7759 + t6483 * t7003 + 2.0_f64 * t100949 + t100952 + 2.0_f64 * t100960 + t86579 + t91813 + t91816 + t91818 + t91824;
    t100962
}
