//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 981/1203 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk981(t33: f64, t32088: f64, t33544: f64, t57: f64, t606: f64, t8960: f64, t32790: f64, t118: f64, t1310: f64, t2127: f64, t2163: f64, t32118: f64, t32123: f64, t32131: f64, t32182: f64, t32299: f64, t32320: f64, t32338: f64, t32340: f64, t32856: f64, t32858: f64, t32862: f64, t32864: f64, t32867: f64, t33375: f64, t508: f64, t649: f64, t7584: f64, t7683: f64, t8917: f64, t8964: f64, dens_threshold: f64, rho1: f64, zeta_threshold: f64) -> (f64, f64) {
    let t34 = t33 <= zeta_threshold;
    let t400 = rho1 <= dens_threshold || t34;
    let t33549 = piecewise3(t400, t32088, t33544 * t57 / 2.0_f64 - t8960 * t606 / 2.0_f64);
    let t33550 = t32790 + t33549;
    let t33552 = -t118 * t33550 - t1310 * t8917 - 2.0_f64 * t2127 * t7683 - 2.0_f64 * t2163 * t7584 - t33375 * t508 - t649 * t8964 - t32118 - t32123 + t32131 + t32182 + t32299 - t32320 - t32338 - t32340 - 4.0_f64 * t32856 - 4.0_f64 * t32858 - 4.0_f64 * t32862 - 4.0_f64 * t32864 - 4.0_f64 * t32867;
    (t33550, t33552)
}
