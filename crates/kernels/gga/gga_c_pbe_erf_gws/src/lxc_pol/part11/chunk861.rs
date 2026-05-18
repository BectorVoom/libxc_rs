//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 861/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk861<F: Float>(t3781: F, t3786: F, t850: F, t860: F, t9144: F, t1109: F, t1134: F, t858: F, t3065: F, t8978: F, t11414: F, t9016: F) -> (F, F, F, F, F, F, F, F) {
    let t13518 = t850 * t3781 * t3786;
    let t13520 = t13518 * t860 / F::new(48.0);
    let t13522 = F::new(35.0) / F::new(144.0) * t9144;
    let t13523 = t1134 * t1109;
    let t13524 = t858 * t13523;
    let t13525 = t3065 * t13524;
    let t13527 = t8978 * t13525 / F::new(32.0);
    let t13529 = t9016 * t11414 / F::new(8.0);
    (t13518, t13520, t13522, t13523, t13524, t13525, t13527, t13529)
}
