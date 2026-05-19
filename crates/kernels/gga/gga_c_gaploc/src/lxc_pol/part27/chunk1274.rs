//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1274/1468 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1274<F: Float>(t33585: F, t2028: F, t2536: F, t787: F, t8632: F, t10007: F, t10627: F, t15482: F, t22628: F, t10930: F, t10931: F, t32893: F) -> (F, F, F, F) {
    let t33586 = F::cast_from(0.19171462976960374838e1_f64) * t33585;
    let t33590 = F::cast_from(0.79445533226334281486e-1_f64) * t787 * t2536 * t8632 * t2028;
    let t33601 = t10007 * t10627;
    let t33604 = F::cast_from(0.22721733898619703511e0_f64) * t22628 * t15482 * t33601;
    let t33607 = F::cast_from(0.27606906686822939767e2_f64) * t10930 * t10931 * t32893;
    (t33586, t33590, t33604, t33607)
}
