//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1110/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1110(t14347: f64, t18648: f64, t18657: f64, t4565: f64, t1662: f64, t3269: f64, t4625: f64, t1670: f64, t4621: f64, t3274: f64, t4670: f64, t1727: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t18713 = t14347 * t18648;
    let t18716 = t4565 * t18657;
    let t18720 = t3269 * t1662 * t4625;
    let t18724 = t3269 * t4621 * t1670;
    let t18728 = t3274 * t1662 * t4670;
    let t18732 = t3274 * t4621 * t1727;
    (t18713, t18716, t18720, t18724, t18728, t18732)
}
