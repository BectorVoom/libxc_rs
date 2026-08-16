//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 2014/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk2014<F: Float>(t90807: F, t90837: F, t93473: F, t93476: F, t93483: F, t93488: F, t93489: F, t93490: F, t93491: F, t93494: F, t96935: F, t96937: F, t96941: F, t96945: F, t96949: F, t96954: F, t96958: F) -> F {
    let t102558 = F::cast_from(0.6579736267392905746e-1_f64) * t96935 - F::cast_from(0.76763589786250567037e-1_f64) * t96937 - t93473 + t93476 - F::cast_from(0.16449340668482264365e-1_f64) * t96941 + t93483 - t93488 + t93489 + t93490 + t93491 + t93494 - F::cast_from(0.5117572652416704469e0_f64) * t90807 + F::cast_from(0.38381794893125283518e-1_f64) * t96945 - F::cast_from(0.16449340668482264365e-1_f64) * t96949 + F::cast_from(0.9869604401089358619e-1_f64) * t96954 - F::cast_from(0.3289868133696452873e-1_f64) * t96958 - F::cast_from(0.20835831513410868196e0_f64) * t90837;
    t102558
}
