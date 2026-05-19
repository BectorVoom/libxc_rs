//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1195/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1195<F: Float>(t13746: F, t13748: F, t13750: F, t13754: F, t13771: F, t16233: F, t16238: F, t16241: F, t16244: F, t16249: F, t16253: F, t16255: F, t16264: F, t16274: F) -> F {
    let t21735 = -F::new(0.1956e1) * t16233 - F::new(0.7335e0) * t16238 + F::new(0.489e0) * t16241 + F::new(0.2445e0) * t16244 - F::new(0.2445e1) * t16249 + F::new(0.9128e1) * t16253 + F::new(0.5868e1) * t16255 - F::new(0.1956e1) * t16264 - F::new(0.3912e1) * t16274 - t13746 + F::cast_from(0.76066666666666666666e1_f64) * t13748 + F::new(0.2282e1) * t13750 - F::new(0.1141e1) * t13754 - F::new(0.2445e1) * t13771;
    t21735
}
