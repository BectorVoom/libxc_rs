//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 356/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk356<F: Float>(t1825: F, t492: F, t83: F, t1544: F, t1548: F, t1551: F, t1562: F, t1567: F, t1574: F, t1583: F, t1591: F, t1758: F, t1769: F, t1810: F) -> (F, F, F) {
    let t1826 = t1825 * t492;
    let t1827 = t83 * t1826;
    let t1832 = F::new(4.0) / F::new(27.0) * t1544;
    let t1841 = -t1769 / F::new(12.0) + t1810 / F::new(6.0) + t1832 + F::new(2.0) / F::new(27.0) * t1548 + F::new(2.0) / F::new(9.0) * t1551 - F::new(2.0) / F::new(27.0) * t1562 + F::new(2.0) / F::new(9.0) * t1567 + F::new(2.0) / F::new(9.0) * t1574 - t1583 / F::new(9.0) + F::new(2.0) / F::new(3.0) * t1591 - t1758 / F::new(3.0);
    (t1826, t1827, t1841)
}
