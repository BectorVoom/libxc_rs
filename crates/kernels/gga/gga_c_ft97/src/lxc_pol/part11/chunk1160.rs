//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1160/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1160<F: Float>(t10491: F, t871: F, t870: F, t9577: F, t2770: F, t2832: F, t10443: F, t10509: F, t10516: F, t10763: F, t1901: F, t2405: F, t2857: F, t2867: F, t2874: F, t2877: F, t319: F, t4139: F, t41691: F, t41698: F, t41718: F, t4265: F, t44210: F, t44219: F, t44518: F, t44523: F, t446: F, t684: F, t835: F, t882: F, t9587: F) -> F {
    let t44528 = t10491 * t871;
    let t44533 = t870 * t9577;
    let t44538 = t2770 * t2832;
    let t44549 = F::new(8.0) / F::new(3.0) * t446 * t835 * t319 * t41691 - F::new(8.0) / F::new(3.0) * t446 * t835 * t882 * t9587 - F::new(8.0) / F::new(3.0) * t446 * t2857 * t319 * t41718 + F::new(2.0) / F::new(3.0) * t446 * t835 * t319 * t41698 - F::new(8.0) / F::new(9.0) * t1901 * t44518 * t2867 * t2405 + F::new(8.0) / F::new(3.0) * t1901 * t44523 * t10763 * t684 + F::new(8.0) / F::new(3.0) * t1901 * t44528 * t10516 * t684 - F::new(16.0) / F::new(9.0) * t1901 * t4139 * t44533 * t44210 + F::new(4.0) / F::new(3.0) * t1901 * t44538 * t2877 - F::new(8.0) / F::new(3.0) * t1901 * t10443 * t10509 - F::new(4.0) / F::new(3.0) * t1901 * t2874 * t4265 * t44219;
    t44549
}
