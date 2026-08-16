//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1115/1304 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1115(t265: f64, t394: f64, t7458: f64, t8675: f64, t1873: f64, t8103: f64, t652: f64, t191: f64, t192: f64, t8107: f64, t2020: f64, t7688: f64, t8690: f64, t33043: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t395 = t265 < t394;
    let t33733 = t7458 * t8675;
    let t33735 = t8103 * t1873;
    let t33736 = t652 * t33735;
    let t33746 = t8107 * t191 * t192;
    let t33747 = t33746 * t2020;
    let t33748 = t8690 * t7688;
    let t33750 = piecewise3(t395, 0.0_f64, t33043);
    (t33733, t33735, t33736, t33746, t33747, t33748, t33750)
}
