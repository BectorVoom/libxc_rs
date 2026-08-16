//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 1022/1059 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk1022(t40889: f64, t8733: f64, t114945: f64, t114965: f64, t116654: f64, t116709: f64, t121745: f64, t121749: f64, t121753: f64, t13463: f64, t1528: f64, t2054: f64, t25168: f64, t2597: f64, t26700: f64, t26713: f64, t2713: f64, t31999: f64, t32002: f64, t33974: f64, t4147: f64, t4268: f64, t4272: f64, t7107: f64, t8734: f64, t92439: f64) -> f64 {
    let t123699 = t40889 * t8733;
    let t123711 = -t4268 * t31999 - 2.0_f64 * t26700 * t7107 + t116654 - t2713 * t33974 - 2.0_f64 * t26713 * t7107 + 0.76763589786250567037e-1_f64 * t114945 + 2.0_f64 * t13463 * t8734 - 0.19739208802178717238e0_f64 * t121745 - 0.16449340668482264365e-1_f64 * t121749 + 24.0_f64 * t25168 * t123699 * t4272 + 4.0_f64 * t4147 * t32002 + 0.16449340668482264365e-1_f64 * t121753 - t2597 * t33974 - 2.0_f64 * t92439 * t2054 + 0.16449340668482264365e-1_f64 * t114965 - t116709 * t1528;
    t123711
}
