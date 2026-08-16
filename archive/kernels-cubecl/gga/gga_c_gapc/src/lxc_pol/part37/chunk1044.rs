//! GGA_C_GAPC lxc pol — lxc_pol part 37 (v4rho2sigma2_16) CSE chunk 1044/1445 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part37_v4rho2sigma2_16_chunk1044<F: Float>(t11259: F, t11265: F, t11268: F, t11274: F, t11276: F, t1049: F, t10526: F, t10529: F, t2967: F, t3179: F, t3480: F, t1112: F, t8598: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t12031 = F::cast_from(0.86898242813537603825e-4_f64) * t11259;
    let t12033 = F::cast_from(0.22776267492663374278e-4_f64) * t11265;
    let t12034 = F::cast_from(0.2530696388073708253e-5_f64) * t11268;
    let t12035 = F::cast_from(0.73811977985483157379e-6_f64) * t11274;
    let t12036 = F::cast_from(0.12147342662753799615e-3_f64) * t11276;
    let t12042 = t10526 * t1049;
    let t12043 = t10529 * t2967;
    let t12044 = F::cast_from(2.0_f64) * t12043;
    let t12045 = t3480 * t3179;
    let t12046 = t8598 * t1112;
    (t12031, t12033, t12034, t12035, t12036, t12042, t12043, t12044, t12045, t12046)
}
