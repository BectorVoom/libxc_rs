//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1017/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1017<F: Float>(t10530: F, t1882: F, t10522: F, t10526: F, t10700: F, t2846: F, t8232: F, t313: F, t41743: F, t89: F, t295: F, t41752: F, t10262: F, t10683: F, t10703: F, t10726: F, t10758: F, t1901: F, t2405: F, t2413: F, t2857: F, t2894: F, t319: F, t41753: F, t41757: F, t446: F, t684: F, t835: F, t871: F, t875: F, t882: F, t9572: F, t9596: F) -> (F, F, F, F) {
    let t44393 = t1882 * t10530;
    let t44395 = t1882 * t10522;
    let t44397 = t1882 * t10526;
    let t44426 = t1882 * t10700;
    let t44428 = t8232 * t2846;
    let t44436 = 280.0 / 243.0 * t89 * t41743 * t313;
    let t44445 = t41752 * t295;
    let t44467 = 8.0 / 3.0 * t44426 + 16.0 / 9.0 * t44428 - 4.0 / 3.0 * t1901 * t10703 * t10726 * t684 + t44436 - 40.0 / 81.0 * t446 * t10758 * t882 * t9572 - t446 * t835 * t319 * t41757 / 9.0 - 80.0 / 243.0 * t446 * t44445 * t319 * t41753 - 4.0 / 9.0 * t446 * t835 * t882 * t9596 - 4.0 / 9.0 * t446 * t2857 * t2894 * t2405 - 2.0 / 3.0 * t446 * t835 * t2894 * t2413 + 8.0 * t446 * t10683 * t871 * t10262 * t875;
    (t44393, t44395, t44397, t44467)
}
