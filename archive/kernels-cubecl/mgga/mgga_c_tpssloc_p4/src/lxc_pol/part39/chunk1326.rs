//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 39 (v4rho3tau_3) CSE chunk 1326/1328 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part39_v4rho3tau_3_chunk1326<F: Float>(t2205: F, t5381: F, t30395: F, t576: F, t2212: F, t5363: F, t1395: F, t8299: F, t110274: F, t110276: F, t110284: F, t111215: F, t1852: F, t3: F, t30133: F, t3932: F, t3946: F, t5364: F, t580: F, t8200: F, t8217: F, t8284: F) -> F {
    let t111302 = F::cast_from(2.0_f64) * t2205 * t5381;
    let t111308 = F::cast_from(2.0_f64) * t576 * t30395;
    let t111310 = F::cast_from(2.0_f64) * t5363 * t2212;
    let t111312 = F::cast_from(2.0_f64) * t1395 * t8299;
    let t111314 = t111215 * t3 * t580 + t1852 * t30133 + t3932 * t8299 + t3946 * t8284 + F::cast_from(2.0_f64) * t5364 * t8217 + F::cast_from(2.0_f64) * t5381 * t8200 + t110274 + t110276 + t110284 + t111302 + t111308 + t111310 + t111312;
    t111314
}
