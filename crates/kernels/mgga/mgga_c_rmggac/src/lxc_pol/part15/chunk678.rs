//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 678/1110 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk678<F: Float>(t675: F, t9795: F, t1743: F, t649: F, t27: F, t2139: F, t1756: F, t2145: F, t1734: F, t2134: F, t2333: F, t8368: F) -> (F, F, F, F, F, F, F, F) {
    let t9796 = t675 * t9795;
    let t9797 = F::cast_from(0.12769379967989351819e-4_f64) * t9796;
    let t9798 = t649 * t1743;
    let t9799 = t27 * t9798;
    let t9800 = t2139 * t9799;
    let t9801 = F::cast_from(0.13637330827122670864e-1_f64) * t9800;
    let t9802 = t649 * t1756;
    let t9803 = t27 * t9802;
    let t9804 = t2145 * t9803;
    let t9805 = F::cast_from(0.34093327067806677161e-2_f64) * t9804;
    let t9806 = t649 * t1734;
    let t9807 = t27 * t9806;
    let t9808 = t2134 * t9807;
    let t9809 = F::cast_from(0.10227998120342003148e-1_f64) * t9808;
    let t9810 = t8368 * t2333;
    (t9797, t9799, t9801, t9803, t9805, t9807, t9809, t9810)
}
