//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1167/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1167<F: Float>(t6944: F, t7979: F, t1600: F, t6937: F, t4432: F, t20: F, t251: F, t7052: F, t1592: F, t2260: F, t27567: F, t27583: F, t27653: F, t28721: F, t29338: F, t29341: F, t29355: F, t29514: F, t29569: F, t29575: F, t29578: F, t29583: F, t7968: F, t7978: F, t8213: F) -> (F, F, F, F, F, F, F) {
    let t29590 = t7979 * t6944;
    let t29591 = t1600 * t29590;
    let t29594 = t7979 * t6937;
    let t29595 = t4432 * t29594;
    let t29599 = t251 * t7052 * t20;
    let t29600 = t1592 * t29599;
    let t29604 = F::new(0.92754700520833333334e-4) * t28721 * t8213 - F::new(0.69505208333333333334e-3) * t7978 * t29569 - F::new(0.13913205078125e-3) * t7968 * t29514 + F::new(0.30918233506944444444e-4) * t27567 * t29575 - F::new(0.34752604166666666667e-3) * t29578 * t2260 + F::new(0.23168402777777777778e-3) * t27583 * t29583 + F::new(0.23168402777777777778e-3) * t27583 * t29575 - F::new(0.34822083333333333332e-2) * t29338 + F::new(0.23214722222222222222e-2) * t29341 - F::new(0.11584201388888888889e-3) * t7978 * t29591 - F::new(0.15445601851851851852e-3) * t7978 * t29595 + t27653 - F::new(0.33980324074074074074e-2) * t29600 * t2260 - F::new(0.23214722222222222222e-2) * t29355;
    (t29590, t29591, t29594, t29595, t29599, t29600, t29604)
}
