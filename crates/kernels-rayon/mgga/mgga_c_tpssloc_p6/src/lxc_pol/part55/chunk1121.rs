//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1121/1304 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1121(t34271: f64, t470: f64, t1737: f64, t1748: f64, t2134: f64, t32425: f64, t32441: f64, t32445: f64, t32448: f64, t34260: f64, t34263: f64, t34266: f64, t488: f64, t7326: f64, t8028: f64, t8031: f64, t8875: f64) -> (f64, f64) {
    let t34272 = t470 * t34271;
    let t34277 = -0.32298204875312312685e-2_f64 * t8028 * t8875 + t32425 - 0.40372756094140390856e-3_f64 * t8031 * t8875 - 0.40372756094140390856e-3_f64 * t2134 * t34260 + 0.40372756094140390856e-3_f64 * t7326 * t34263 + t34266 * t488 / 1536.0_f64 + t32441 * t1737 / 1536.0_f64 - t34272 * t488 / 288.0_f64 + t32445 - t32448 * t1748 / 2304.0_f64;
    (t34272, t34277)
}
