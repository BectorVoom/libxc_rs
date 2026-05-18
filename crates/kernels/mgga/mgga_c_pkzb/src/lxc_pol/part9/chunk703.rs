//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 703/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk703<F: Float>(t1447: F, t40: F, t31: F, t1450: F, t459: F, t1466: F, t1418: F, t1426: F, t1432: F, t1453: F, t16: F, t34: F, t38: F, t441: F, t454: F, t4796: F, t4800: F, t4806: F, t4812: F, t4816: F, t4820: F, tau0: F) -> (F, F, F, F, F, F) {
    let t4827 = F::new(1.0) / t1447 / t40;
    let t4828 = t31 * t4827;
    let t4829 = t1450 * t459;
    let t4832 = t459 * t1466;
    let t4835 = tau0 * t1418;
    let t4856 = -F::new(440.0) / F::new(27.0) * t4835 * t16 + F::new(200.0) / F::new(9.0) * t1453 * t441 - F::new(50.0) / F::new(9.0) * t454 * t1426 - F::new(25.0) / F::new(3.0) * t454 * t1432 - F::new(10.0) / F::new(27.0) * t34 * t4796 + F::new(10.0) / F::new(3.0) * t34 * t4800 + F::new(5.0) / F::new(3.0) * t34 * t4806 - F::new(10.0) / F::new(27.0) * t38 * t4812 + F::new(10.0) / F::new(3.0) * t38 * t4816 + F::new(5.0) / F::new(3.0) * t38 * t4820;
    (t4827, t4828, t4829, t4832, t4835, t4856)
}
