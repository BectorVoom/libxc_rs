//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 780/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk780<F: Float>(t24633: F, t24649: F, t241: F, t258: F, t1445: F, t2399: F, t89: F, t2413: F, t6161: F, t2606: F, t1901: F, t193: F, t24567: F, t24571: F, t24575: F, t24579: F, t24583: F, t24587: F, t24590: F, t24592: F, t24596: F, t24601: F, t24605: F, t24608: F, t24611: F, t24614: F, t446: F) -> (F, F, F, F, F, F) {
    let t24650 = t24633 + t24649;
    let t24652 = t241 * t24650 * t258;
    let t24658 = 4.0 / 27.0 * t89 * t2399 * t1445;
    let t24659 = t6161 * t2413;
    let t24660 = t2606 * t24659;
    let t24663 = -4.0 / 9.0 * t24567 - 2.0 / 9.0 * t1901 * t24571 - 2.0 * t446 * t24575 - 2.0 / 9.0 * t1901 * t24579 + 4.0 / 3.0 * t446 * t24583 + 4.0 / 3.0 * t446 * t24587 - 4.0 / 9.0 * t24590 - 2.0 / 9.0 * t24592 - 2.0 / 3.0 * t446 * t24596 + 2.0 / 9.0 * t1901 * t24601 - 2.0 / 9.0 * t24605 + 2.0 / 9.0 * t446 * t24608 - 2.0 / 9.0 * t24611 - t446 * t24614 / 3.0 + t89 * t193 * t24652 / 3.0 + t24658 + t1901 * t24660 / 9.0;
    (t24650, t24652, t24658, t24659, t24660, t24663)
}
