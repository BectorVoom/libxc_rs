//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 451/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk451<F: Float>(t1902: F, t4607: F, t920: F, t979: F, t1910: F, t1909: F, t110: F, t4458: F, t447: F, t1887: F, t1901: F, t28: F, t3177: F, t3224: F, t3260: F, t3286: F, t446: F, t4547: F, t4553: F, t4557: F, t4561: F, t4565: F, t4569: F, t4574: F, t4591: F, t4595: F, t4599: F, t4603: F, t89: F) -> (F, F, F, F, F, F) {
    let t4608 = t1902 * t4607;
    let t4611 = t920 * t979;
    let t4612 = t1910 * t4611;
    let t4613 = t1909 * t4612;
    let t4617 = t447 * t110 * t4458;
    let t4621 = t89 * t28 * t4547 / F::new(3.0) + F::new(2.0) / F::new(3.0) * t446 * t4553 - F::new(2.0) / F::new(9.0) * t446 * t4557 - t446 * t4561 / F::new(9.0) - F::new(2.0) / F::new(27.0) * t446 * t4565 + F::new(2.0) / F::new(3.0) * t446 * t4569 + F::new(2.0) / F::new(3.0) * t446 * t4574 + F::new(2.0) / F::new(9.0) * t3224 + F::new(2.0) / F::new(9.0) * t3260 + t1887 - F::new(2.0) / F::new(9.0) * t3177 - t446 * t4591 / F::new(3.0) - F::new(2.0) / F::new(3.0) * t446 * t4595 - F::new(2.0) / F::new(3.0) * t446 * t4599 - t446 * t4603 / F::new(3.0) + F::new(2.0) / F::new(9.0) * t1901 * t4608 + F::new(2.0) / F::new(9.0) * t1901 * t4613 + F::new(2.0) / F::new(9.0) * t446 * t4617 + F::new(2.0) / F::new(27.0) * t3286;
    (t4608, t4611, t4612, t4613, t4617, t4621)
}
