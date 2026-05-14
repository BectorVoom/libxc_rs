//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1021/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1021<F: Float>(t4973: F, t6074: F, t2599: F, t3977: F, t6861: F, t729: F, t1168: F, t6837: F, t762: F, t13839: F, t6917: F, t1091: F, t28355: F, t2606: F, t24793: F, t5171: F) -> (F, F, F, F, F, F, F, F, F) {
    let t31147 = t6074 * t4973;
    let t31148 = t2599 * t31147;
    let t31152 = t729 * t3977 * t6861;
    let t31155 = t6837 * t1168;
    let t31157 = t729 * t762 * t31155;
    let t31160 = t13839 * t6917;
    let t31163 = t28355 * t1091;
    let t31164 = t2606 * t31163;
    let t31167 = t24793 * t5171;
    (t31147, t31148, t31152, t31155, t31157, t31160, t31163, t31164, t31167)
}
