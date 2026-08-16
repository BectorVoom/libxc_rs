//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1128/1304 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1128(t113: f64, t1459: f64, t1774: f64, t1849: f64, t2114: f64, t2165: f64, t32609: f64, t33096: f64, t33098: f64, t33100: f64, t33131: f64, t33139: f64, t33158: f64, t33162: f64, t33736: f64, t33747: f64, t33748: f64, t33758: f64, t34229: f64, t34372: f64, t34381: f64, t510: f64, t574: f64, t7983: f64, t8103: f64, t8860: f64, t8916: f64) -> f64 {
    let t34384 = -t113 * t34372 - 2.0_f64 * t1459 * t32609 - t1774 * t8860 + t1849 * t8916 - 2.0_f64 * t2114 * t8103 - 2.0_f64 * t2165 * t7983 - t34229 * t510 + t34381 * t574 - t33096 - t33098 - t33100 + t33131 - t33139 - t33158 - t33162 - 4.0_f64 * t33736 + 2.0_f64 * t33747 + 6.0_f64 * t33748 + 2.0_f64 * t33758;
    t34384
}
