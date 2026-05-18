//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1158/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1158<F: Float>(t10262: F, t10683: F, t10703: F, t10726: F, t10758: F, t1901: F, t2405: F, t2413: F, t2857: F, t2894: F, t319: F, t41753: F, t41757: F, t44426: F, t44428: F, t44436: F, t44445: F, t446: F, t684: F, t835: F, t871: F, t875: F, t882: F, t9572: F, t9596: F) -> F {
    let t44467 = F::new(8.0) / F::new(3.0) * t44426 + F::new(16.0) / F::new(9.0) * t44428 - F::new(4.0) / F::new(3.0) * t1901 * t10703 * t10726 * t684 + t44436 - F::new(40.0) / F::new(81.0) * t446 * t10758 * t882 * t9572 - t446 * t835 * t319 * t41757 / F::new(9.0) - F::new(80.0) / F::new(243.0) * t446 * t44445 * t319 * t41753 - F::new(4.0) / F::new(9.0) * t446 * t835 * t882 * t9596 - F::new(4.0) / F::new(9.0) * t446 * t2857 * t2894 * t2405 - F::new(2.0) / F::new(3.0) * t446 * t835 * t2894 * t2413 + F::new(8.0) * t446 * t10683 * t871 * t10262 * t875;
    t44467
}
