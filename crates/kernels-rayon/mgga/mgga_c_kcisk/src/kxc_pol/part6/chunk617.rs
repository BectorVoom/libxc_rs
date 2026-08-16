//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 617/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk617(t4704: f64, t8550: f64, t4716: f64, t8504: f64, t1653: f64, t8522: f64, t4726: f64, t8510: f64, t26: f64, t1659: f64, t8514: f64, t8518: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t8552 = 2.0_f64 * t4704 * t8550;
    let t8559 = t4716 * t8504;
    let t8561 = t1653 * t8522;
    let t8564 = t4726 * t8510;
    let t8565 = t26 * t8564;
    let t8567 = t1659 * t8514;
    let t8568 = t26 * t8567;
    let t8570 = t1659 * t8518;
    (t8552, t8559, t8561, t8564, t8565, t8567, t8568, t8570)
}
