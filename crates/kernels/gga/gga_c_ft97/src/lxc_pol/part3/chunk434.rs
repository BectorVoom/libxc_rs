//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 434/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk434<F: Float>(t3103: F, t370: F, t27: F, t89: F, t1545: F, t1548: F, t1551: F, t2981: F, t2986: F, t2990: F, t2995: F, t3003: F, t3006: F, t3011: F, t3016: F) -> (F, F, F) {
    let t3104 = t370 * t3103;
    let t3106 = t89 * t27 * t3104;
    let t3108 = t1545 + t1548 / F::new(54.0) + t1551 / F::new(18.0) + t2981 / F::new(54.0) - t2986 / F::new(27.0) + t2990 / F::new(18.0) + t2995 / F::new(9.0) - t3003 / F::new(9.0) + t3006 / F::new(18.0) + t3011 / F::new(18.0) + t3016 / F::new(3.0) - t3106 / F::new(6.0);
    (t3104, t3106, t3108)
}
