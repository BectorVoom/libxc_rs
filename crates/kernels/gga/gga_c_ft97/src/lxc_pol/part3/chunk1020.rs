//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 1020/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk1020<F: Float>(t19782: F, t295: F, t312: F, t15290: F, t19006: F, t1901: F, t193: F, t19587: F, t19590: F, t19594: F, t19598: F, t19602: F, t19606: F, t19610: F, t19614: F, t19618: F, t19623: F, t19627: F, t19631: F, t19635: F, t3281: F, t446: F, t89: F) -> F {
    let t19784 = t295 * t19782 * t312;
    let t19788 = t15290 * t19006;
    let t19791 = -F::new(2.0) / F::new(9.0) * t1901 * t19587 + F::new(2.0) / F::new(9.0) * t1901 * t19590 - F::new(2.0) / F::new(9.0) * t1901 * t19594 - F::new(2.0) / F::new(9.0) * t446 * t19598 + F::new(4.0) / F::new(9.0) * t3281 * t19602 - t446 * t19606 / F::new(9.0) - t446 * t19610 / F::new(9.0) - F::new(2.0) / F::new(27.0) * t446 * t19614 - F::new(4.0) / F::new(9.0) * t1901 * t19618 - F::new(4.0) / F::new(9.0) * t1901 * t19623 + F::new(4.0) / F::new(27.0) * t1901 * t19627 - F::new(2.0) / F::new(9.0) * t1901 * t19631 - t19635 / F::new(9.0) + t89 * t193 * t19784 / F::new(3.0) + F::new(4.0) / F::new(27.0) * t1901 * t19788;
    t19791
}
