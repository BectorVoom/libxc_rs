//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 753/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk753<F: Float>(t1073: F, t4454: F, t8654: F, t20022: F, t2258: F, t8660: F, t20031: F, t3613: F, t12165: F, t12204: F, t17552: F, t17554: F, t17556: F, t17573: F, t17626: F, t17627: F, t2265: F, t631: F, t8718: F) -> (F, F, F, F) {
    let t21068 = t8654 * t4454 * t1073;
    let t21072 = t2258 * t8660 * t20022;
    let t21075 = t3613 * t20031;
    let t21085 = -t2265 * t21068 / F::new(3.0) - t631 * t21072 / F::new(3.0) + t2265 * t21075 / F::new(6.0) + F::new(4.0) / F::new(3.0) * t17573 - t17627 + F::new(3.0) * t17626 + F::new(2.0) / F::new(3.0) * t17552 - t17554 / F::new(3.0) - t17556 / F::new(9.0) + F::new(5.0) / F::new(3.0) * t12204 + t8718 + F::new(5.0) / F::new(9.0) * t12165;
    (t21068, t21072, t21075, t21085)
}
