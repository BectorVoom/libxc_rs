//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2185/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2185(t11719: f64, t11722: f64, t248: f64, t3570: f64, t11818: f64, t1213: f64, t3494: f64, t3506: f64, t3509: f64, t3515: f64, t3516: f64, t11718: f64, t44857: f64) -> (f64, f64, f64, f64, f64) {
    let t44871 = t11719 * t248 * t3570 * t11722;
    let t44886 = t1213 * t248 * t11818 * t3494;
    let t44890 = t3506 * t248 * t11818 * t3509;
    let t44894 = t3515 * t248 * t11818 * t3516;
    let t44896 = t44857 * t11718;
    (t44871, t44886, t44890, t44894, t44896)
}
