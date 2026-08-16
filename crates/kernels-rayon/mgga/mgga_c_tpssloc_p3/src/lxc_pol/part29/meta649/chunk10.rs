//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2168/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2168(t1081: f64, t4255: f64, t870: f64, t23788: f64, t58071: f64, t86706: f64, t1649: f64, t2745: f64, t25927: f64, t86713: f64, t2379: f64, t1877: f64, t1915: f64, t22959: f64, t23789: f64, t23792: f64, t25013: f64, t2522: f64, t25372: f64, t4314: f64, t6670: f64, t6848: f64, t7541: f64, t86736: f64, t86836: f64, t89837: f64, t89840: f64, t89843: f64, t89846: f64, t89850: f64) -> f64 {
    let t89859 = t870 * t1081 * t4255;
    let t89862 = t23788 * t58071;
    let t89865 = t23788 * t86706;
    let t89868 = t1649 * t2745;
    let t89872 = t25927 * t86713;
    let t89874 = t1649 * t2379;
    let t89880 = -3.0_f64 / 2.0_f64 * t22959 * t89837 - 3.0_f64 / 2.0_f64 * t22959 * t89840 + 3.0_f64 * t25013 * t89843 + 2.0_f64 * t25372 * t89846 + 2.0_f64 * t25372 * t89850 - 3.0_f64 * t86736 * t23789 + 3.0_f64 * t2522 * t7541 * t23792 + 6.0_f64 * t25013 * t89859 - 3.0_f64 * t22959 * t89862 - 3.0_f64 * t25013 * t89865 - t1877 * t6670 * t89868 / 2.0_f64 + t25372 * t89872 + 3.0_f64 * t4314 * t1915 * t89874 - t1877 * t86836 * t6848;
    t89880
}
