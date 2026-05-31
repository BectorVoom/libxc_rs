//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 848/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk848<F: Float>(t1901: F, t33658: F, t33680: F, t33682: F, t35596: F, t35601: F, t35606: F, t35610: F, t35614: F, t35617: F, t35621: F, t35625: F, t35629: F, t446: F) -> F {
    let t35632 = t33658 + t446 * t35596 / F::cast_from(3.0_f64) - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t35601 - F::cast_from(2.0_f64) * t446 * t35606 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t1901 * t35610 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t1901 * t35614 + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t1901 * t35617 + t1901 * t35621 / F::cast_from(9.0_f64) - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t1901 * t35625 - t33680 + t33682 - t446 * t35629 / F::cast_from(3.0_f64);
    t35632
}
