//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1143/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1143<F: Float>(t762: F, t88192: F, t88215: F, t89044: F, t89089: F, t5147: F, t2568: F, t10007: F, t10157: F, t1091: F, t1175: F, t14200: F, t1901: F, t21416: F, t21486: F, t242: F, t265: F, t446: F, t729: F, t80345: F, t80399: F, t80406: F, t80412: F, t80429: F, t88196: F, t88939: F) -> (F, F, F) {
    let t89092 = t762 * (t88192 + t88215 + t89044 + t89089);
    let t89096 = t5147 * t5147;
    let t89097 = t2568 * t89096;
    let t89117 = -F::new(8.0) * t446 * t10157 * t1175 * t21416 - t446 * t242 * t89092 / F::new(3.0) + F::new(2.0) * t446 * t242 * t89097 + F::new(8.0) / F::new(9.0) * t80345 - t446 * t729 * t265 * t88939 / F::new(3.0) - F::new(4.0) / F::new(3.0) * t80399 - F::new(4.0) / F::new(9.0) * t80406 + F::new(8.0) / F::new(9.0) * t1901 * t14200 * t88196 - F::new(4.0) / F::new(3.0) * t1901 * t10007 * t21486 * t1091 + F::new(8.0) / F::new(9.0) * t80412 - F::new(8.0) / F::new(27.0) * t80429;
    (t89092, t89097, t89117)
}
