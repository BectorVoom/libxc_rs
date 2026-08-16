//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1127/1304 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1127(t28: f64, t1409: f64, t33073: f64, t34366: f64, t52: f64, t8909: f64, t33755: f64, t1458: f64, t32609: f64, t33148: f64, t33150: f64, t33152: f64, t33154: f64, t33711: f64, t33713: f64, t33715: f64, t34229: f64, t8446: f64, dens_threshold: f64, rho1: f64, zeta_threshold: f64) -> (f64, f64) {
    let t29 = t28 <= zeta_threshold;
    let t401 = rho1 <= dens_threshold || t29;
    let t34371 = piecewise3(t401, t33073, -t8909 * t1409 / 2.0_f64 + t34366 * t52 / 2.0_f64);
    let t34372 = t33755 + t34371;
    let t34381 = 2.0_f64 * t1458 * t32609 + t33148 + t33150 + t33152 + t33154 + 4.0_f64 * t33711 + 4.0_f64 * t33713 + 4.0_f64 * t33715 + t34229 + t8446;
    (t34372, t34381)
}
