//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1316/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1316<F: Float>(t11057: F, t28737: F, t10942: F, t28673: F, t2028: F, t2536: F, t787: F, t8632: F, t10007: F, t10627: F, t15482: F, t22628: F) -> (F, F, F, F) {
    let t33583 = t28737 * t11057;
    let t33584 = F::cast_from(0.76685851907841499352e0_f64) * t33583;
    let t33585 = t28673 * t10942;
    let t33586 = F::cast_from(0.19171462976960374838e1_f64) * t33585;
    let t33590 = F::cast_from(0.79445533226334281486e-1_f64) * t787 * t2536 * t8632 * t2028;
    let t33601 = t10007 * t10627;
    let t33604 = F::cast_from(0.22721733898619703511e0_f64) * t22628 * t15482 * t33601;
    (t33584, t33586, t33590, t33604)
}
