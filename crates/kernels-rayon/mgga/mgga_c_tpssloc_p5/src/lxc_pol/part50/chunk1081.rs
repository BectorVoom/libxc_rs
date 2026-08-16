//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1081/1294 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1081(t32677: f64, t652: f64, t1874: f64, t24999: f64, t7685: f64, t8490: f64, t1842: f64, t8485: f64, t3887: f64, t8475: f64, t12021: f64, t31090: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t32678 = t652 * t32677;
    let t32679 = 2.0_f64 * t32678;
    let t32680 = t24999 * t1874;
    let t32684 = t7685 * t8490;
    let t32685 = t8485 * t1842;
    let t32686 = t3887 * t32685;
    let t32689 = t8475 * t1842;
    let t32690 = t12021 * t32689;
    let t32693 = t31090 * t1842;
    (t32679, t32680, t32684, t32686, t32690, t32693)
}
