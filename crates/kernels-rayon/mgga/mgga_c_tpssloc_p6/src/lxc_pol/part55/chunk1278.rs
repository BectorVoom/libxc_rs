//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1278/1304 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1278(t118006: f64, t27608: f64, t32446: f64, t373: f64, t471: f64, t10401: f64, t117949: f64, t117954: f64, t117977: f64, t119243: f64, t1734: f64, t1748: f64, t24685: f64, t24727: f64, t27629: f64, t27636: f64, t27638: f64, t27644: f64, t32429: f64, t32433: f64, t32448: f64, t34263: f64, t3500: f64, t4950: f64, t4954: f64, t4979: f64, t4988: f64, t5030: f64, t7337: f64, t8028: f64) -> (f64, f64) {
    let t125443 = t27608 * t118006;
    let t125453 = t471 * t32446 * t373;
    let t125459 = -t117949 * t1748 / 2304.0_f64 - t32448 * t5030 / 2304.0_f64 - 0.40372756094140390856e-3_f64 * t24685 * t34263 + 0.80745512188280781712e-3_f64 * t27636 * t24727 * t1734 * t27638 - 0.40372756094140390856e-3_f64 * t27636 * t7337 * t1734 * t27644 - t117977 * t4950 / 2304.0_f64 - t117977 * t4954 / 2304.0_f64 - 0.40372756094140390856e-3_f64 * t125443 - 0.40372756094140390856e-3_f64 * t27629 * t32433 + t3500 * t117954 * t10401 * t119243 * t4979 / 768.0_f64 + 5.0_f64 / 6912.0_f64 * t125453 * t119243 * t4988 + 0.32298204875312312685e-2_f64 * t8028 * t32429;
    (t125453, t125459)
}
