//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1260/1468 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1260<F: Float>(t10930: F, t10931: F, t32803: F, t16687: F, t19: F, t60: F, t822: F, t16692: F, t201: F, t2679: F, t2963: F, t9796: F) -> (F, F, F) {
    let t33328 = F::cast_from(0.38649669361552115674e3_f64) * t10930 * t10931 * t32803;
    let t33331 = t822 * t16687 * t19 * t60;
    let t33332 = t201 * t16692;
    let t33335 = F::cast_from(0.13803453343411469884e3_f64) * t33331 * t33332 * t32803;
    let t33337 = t9796 * t2963 * t2679;
    (t33328, t33335, t33337)
}
