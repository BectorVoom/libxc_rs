//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1227/1400 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1227(t1873: f64, t27863: f64, t33690: f64, t7266: f64, t7467: f64, t1458: f64, t31880: f64, t33142: f64, t33144: f64, t33146: f64, t33148: f64, t33150: f64, t33152: f64, t33154: f64, t33686: f64, t8446: f64) -> f64 {
    let t33711 = t27863 * t1873;
    let t33713 = t33690 * t1873;
    let t33715 = t7266 * t7467;
    let t33720 = 2.0_f64 * t1458 * t31880 + 2.0_f64 * t33142 + 2.0_f64 * t33144 + 2.0_f64 * t33146 + t33148 + t33150 + t33152 + t33154 + t33686 + 2.0_f64 * t33711 + 2.0_f64 * t33713 + 2.0_f64 * t33715 + t8446;
    t33720
}
