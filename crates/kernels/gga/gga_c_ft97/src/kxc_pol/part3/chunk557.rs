//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 557/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk557<F: Float>(t378: F, t4462: F, t92: F, t1639: F, t3042: F, t4456: F, t4460: F) -> (F, F, F) {
    let t4463 = t378 * t4462;
    let t4464 = t92 * t4463;
    let t4466 = t1639 + F::new(2.0) / F::new(9.0) * t3042 - F::new(2.0) / F::new(9.0) * t4456 + F::new(2.0) / F::new(3.0) * t4460 - t4464 / F::new(3.0);
    (t4463, t4464, t4466)
}
