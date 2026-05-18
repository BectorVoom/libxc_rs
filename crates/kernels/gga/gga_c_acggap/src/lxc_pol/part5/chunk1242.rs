//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1242/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1242<F: Float>(t1454: F, t372: F, t1131: F, t1143: F, t1165: F, t1181: F, t13654: F, t13851: F, t1501: F, t1532: F, t1734: F, t17411: F, t17421: F, t17430: F, t17436: F, t17441: F, t1748: F, t1894: F, t335: F, t336: F, t3396: F, t3565: F, t367: F, t4099: F, t4876: F, t513: F, t5506: F, t6138: F, t922: F) -> (F, F) {
    let t22778 = t1454 * t372;
    let t22787 = F::new(0.45351183609335988442e0) * t17411 + F::new(0.17149607247227894789e-2) * t17421 - t335 * t336 * t1143 * t5506 / F::new(24.0) - t367 * t336 * t1894 * t1131 / F::new(96.0) - t367 * t336 * t4876 * t513 / F::new(48.0) - t335 * t336 * t1501 * t4099 / F::new(24.0) - t335 * t336 * t3565 * t1734 / F::new(48.0) + F::new(0.51448821741683684367e-1) * t13851 * t1165 * t1532 * t1748 * t922 - F::new(0.41159057393346947493e-1) * t3396 * t1181 * t6138 * t22778 + F::new(0.68598428988911579156e-2) * t17430 + F::new(35.0) / F::new(108.0) * t13654 - F::new(0.17149607247227894789e-2) * t17436 - F::new(0.17149607247227894789e-2) * t17441;
    (t22778, t22787)
}
