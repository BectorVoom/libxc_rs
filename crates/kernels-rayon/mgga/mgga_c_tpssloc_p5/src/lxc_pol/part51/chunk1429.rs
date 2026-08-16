//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1429/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1429(t120297: f64, t120304: f64, t122204: f64, t122206: f64, t122210: f64, t122213: f64, t122218: f64, t16460: f64, t2092: f64, t26224: f64, t26226: f64, t26481: f64, t26989: f64, t27062: f64, t33294: f64, t33320: f64, t3758: f64, t6958: f64, t8637: f64, t91488: f64) -> f64 {
    let t122223 = -6.0_f64 * t26224 * t26989 * t26481 + 2.0_f64 * t6958 * t27062 + 0.16449340668482264365e-1_f64 * t122204 - 6.0_f64 * t122206 * t26226 - t3758 * t33294 + t120297 + 0.19190897446562641759e-1_f64 * t122210 + 0.16449340668482264365e-1_f64 * t122213 - t91488 * t2092 - 0.3289868133696452873e-1_f64 * t122218 + t120304 + 2.0_f64 * t3758 * t33320 - t16460 * t8637;
    t122223
}
