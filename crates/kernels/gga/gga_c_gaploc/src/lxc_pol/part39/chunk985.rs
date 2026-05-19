//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 985/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk985<F: Float>(t42933: F, t3247: F, t32692: F, t9647: F, t10697: F, t9624: F, t2558: F, t33348: F, t1022: F, t3209: F) -> (F, F, F, F, F) {
    let t42934 = F::cast_from(0.1922631557535556071e-2_f64) * t42933;
    let t42936 = t9647 * t32692 * t3247;
    let t42937 = F::cast_from(0.1922631557535556071e-2_f64) * t42936;
    let t42939 = t9647 * t10697 * t9624;
    let t42940 = F::cast_from(0.1922631557535556071e-2_f64) * t42939;
    let t42942 = t9647 * t33348 * t2558;
    let t42943 = F::cast_from(0.64087718584518535698e-3_f64) * t42942;
    let t42944 = t1022 * t3209;
    (t42934, t42937, t42940, t42943, t42944)
}
