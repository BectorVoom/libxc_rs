//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 944/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk944<F: Float>(t18587: F, t241: F, t258: F, t1882: F, t5153: F, t2574: F, t4934: F, t773: F, t11593: F, t14126: F, t14138: F, t18516: F, t18521: F, t18526: F, t18529: F, t18534: F, t18538: F, t18540: F, t18542: F, t18544: F, t1901: F, t193: F, t446: F, t89: F, t9982: F) -> F {
    let t18589 = t241 * t18587 * t258;
    let t18593 = t1882 * t5153;
    let t18596 = t2574 * t773 * t4934;
    let t18599 = -F::new(2.0) / F::new(3.0) * t1901 * t18516 - F::new(4.0) / F::new(9.0) * t11593 * t18521 - F::new(4.0) / F::new(9.0) * t11593 * t18526 + F::new(2.0) / F::new(9.0) * t1901 * t18529 + F::new(2.0) / F::new(9.0) * t1901 * t18534 - t9982 - t14126 - F::new(4.0) / F::new(27.0) * t14138 + F::new(2.0) / F::new(81.0) * t18538 + t18540 / F::new(27.0) + F::new(2.0) / F::new(27.0) * t18542 - F::new(2.0) / F::new(9.0) * t18544 + t89 * t193 * t18589 / F::new(3.0) + F::new(2.0) / F::new(9.0) * t18593 + F::new(2.0) / F::new(3.0) * t446 * t18596;
    t18599
}
