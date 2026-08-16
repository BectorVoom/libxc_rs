//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 812/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk812(t25: f64, t265: f64, t394: f64, t29148: f64, t1409: f64, t2064: f64, t29124: f64, t40: f64, t5398: f64, t7865: f64, t2057: f64, t28764: f64, t1649: f64, t1877: f64, t24191: f64, t24344: f64, t2522: f64, t26744: f64, t28: f64, t28771: f64, t28774: f64, t28778: f64, t28789: f64, t28792: f64, t28795: f64, t29106: f64, t4314: f64, t5966: f64, t7114: f64, t7649: f64, t7656: f64, t7845: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64) -> (f64, f64) {
    let t26 = t25 <= zeta_threshold;
    let t115 = rho0 <= dens_threshold || t26;
    let t395 = t265 < t394;
    let t29149 = piecewise3(t395, 0.0_f64, t29148);
    let t29156 = piecewise3(t115, t29124, t29149 * t40 / 2.0_f64 + t7865 * t1409 + t2064 * t5398 / 2.0_f64);
    let t29157 = t2057 * t28764;
    let t29188 = 3.0_f64 * t4314 * t29157 + 3.0_f64 * t2522 * t7845 * t7649 - 3.0_f64 * t24191 * t28771 + 3.0_f64 * t2522 * t2057 * t28774 + 3.0_f64 / 2.0_f64 * t2522 * t2057 * t28778 + t1877 * t29106 * t28 / 2.0_f64 - t1877 * t26744 * t7656 + t1877 * t7845 * t1649 + t1877 * t24344 * t28789 - t1877 * t7114 * t28792 - t1877 * t7114 * t28795 / 2.0_f64 + t1877 * t2057 * t5966 / 2.0_f64;
    (t29156, t29188)
}
