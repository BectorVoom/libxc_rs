//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 782/1034 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk782(t25: f64, t265: f64, t394: f64, t24379: f64, t2064: f64, t2250: f64, t24355: f64, t40: f64, t607: f64, t7131: f64, t1081: f64, t1877: f64, t2057: f64, t23781: f64, t23789: f64, t23792: f64, t23796: f64, t23807: f64, t23810: f64, t23813: f64, t24191: f64, t24335: f64, t24339: f64, t24344: f64, t2522: f64, t28: f64, t3231: f64, t4314: f64, t6841: f64, t6848: f64, t7110: f64, t7114: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64) -> (f64, f64) {
    let t26 = t25 <= zeta_threshold;
    let t115 = rho0 <= dens_threshold || t26;
    let t395 = t265 < t394;
    let t24380 = piecewise3(t395, 0.0_f64, t24379);
    let t24387 = piecewise3(t115, t24355, t24380 * t40 / 2.0_f64 + t7131 * t607 + t2064 * t2250 / 2.0_f64);
    let t24419 = 3.0_f64 * t4314 * t2057 * t23781 + 3.0_f64 * t2522 * t7110 * t6841 - 3.0_f64 * t24191 * t23789 + 3.0_f64 * t2522 * t2057 * t23792 + 3.0_f64 / 2.0_f64 * t2522 * t2057 * t23796 + t1877 * t24335 * t28 / 2.0_f64 - t1877 * t24339 * t6848 + t1877 * t7110 * t1081 + t1877 * t24344 * t23807 - t1877 * t7114 * t23810 - t1877 * t7114 * t23813 / 2.0_f64 + t1877 * t2057 * t3231 / 2.0_f64;
    (t24387, t24419)
}
