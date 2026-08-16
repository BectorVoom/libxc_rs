//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1167/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1167(t6944: f64, t7979: f64, t1600: f64, t6937: f64, t4432: f64, t20: f64, t251: f64, t7052: f64, t1592: f64, t2260: f64, t27567: f64, t27583: f64, t27653: f64, t28721: f64, t29338: f64, t29341: f64, t29355: f64, t29514: f64, t29569: f64, t29575: f64, t29578: f64, t29583: f64, t7968: f64, t7978: f64, t8213: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t29590 = t7979 * t6944;
    let t29591 = t1600 * t29590;
    let t29594 = t7979 * t6937;
    let t29595 = t4432 * t29594;
    let t29599 = t251 * t7052 * t20;
    let t29600 = t1592 * t29599;
    let t29604 = 0.92754700520833333334e-4_f64 * t28721 * t8213 - 0.69505208333333333334e-3_f64 * t7978 * t29569 - 0.13913205078125e-3_f64 * t7968 * t29514 + 0.30918233506944444444e-4_f64 * t27567 * t29575 - 0.34752604166666666667e-3_f64 * t29578 * t2260 + 0.23168402777777777778e-3_f64 * t27583 * t29583 + 0.23168402777777777778e-3_f64 * t27583 * t29575 - 0.34822083333333333332e-2_f64 * t29338 + 0.23214722222222222222e-2_f64 * t29341 - 0.11584201388888888889e-3_f64 * t7978 * t29591 - 0.15445601851851851852e-3_f64 * t7978 * t29595 + t27653 - 0.33980324074074074074e-2_f64 * t29600 * t2260 - 0.23214722222222222222e-2_f64 * t29355;
    (t29590, t29591, t29594, t29595, t29599, t29600, t29604)
}
