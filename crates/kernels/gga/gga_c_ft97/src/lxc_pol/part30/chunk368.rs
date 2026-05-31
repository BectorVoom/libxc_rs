//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 368/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk368<F: Float>(t6187: F, t762: F, t242: F, t1901: F, t193: F, t446: F, t6073: F, t6076: F, t6081: F, t6085: F, t6090: F, t6094: F, t6099: F, t6101: F, t6105: F, t6150: F, t6156: F, t6160: F, t6163: F, t6168: F, t6172: F, t6177: F, t89: F) -> (F, F) {
    let t6188 = t762 * t6187;
    let t6189 = t242 * t6188;
    let t6192 = t6073 + t1901 * t6076 / F::cast_from(9.0_f64) + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t6081 - t446 * t6085 / F::cast_from(3.0_f64) + t446 * t6090 / F::cast_from(3.0_f64) - t446 * t6094 / F::cast_from(3.0_f64) - t6099 - t446 * t6101 / F::cast_from(9.0_f64) - t446 * t6105 / F::cast_from(3.0_f64) + t89 * t193 * t6150 / F::cast_from(3.0_f64) - t446 * t6156 / F::cast_from(3.0_f64) + t6160 + t1901 * t6163 / F::cast_from(9.0_f64) + t446 * t6168 / F::cast_from(3.0_f64) - t446 * t6172 / F::cast_from(3.0_f64) + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t6177 - t446 * t6189 / F::cast_from(3.0_f64);
    (t6189, t6192)
}
