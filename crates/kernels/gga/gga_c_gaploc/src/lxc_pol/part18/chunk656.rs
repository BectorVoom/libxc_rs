//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 656/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk656<F: Float>(t1683: F, t283: F, t1691: F, t458: F, t711: F, t291: F, t1842: F, t486: F, t701: F, t723: F) -> (F, F, F, F, F, F, F, F, F) {
    let pi = F::cast_from(M_PI);
    let t5335 = t1683 * t1683;
    let t5337 = F::new(1.0) / t5335 / t283;
    let t5340 = pi * t1691 * t458;
    let t5343 = t711 * t711;
    let t5344 = F::new(1.0) / t5343;
    let t5345 = t291 * t5344;
    let t5348 = t5337 * pi * t458;
    let t5396 = t1842 * t486;
    let t5397 = t723 * t701;
    (t5335, t5337, t5340, t5343, t5344, t5345, t5348, t5396, t5397)
}
