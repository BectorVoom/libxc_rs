//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1397/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1397<F: Float>(t3869: F, t39538: F, t39427: F, t39535: F, t3853: F, t3857: F, t73: F, t9940: F, t820: F, t843: F, t9991: F, t1386: F, t2237: F, t2482: F) -> (F, F, F, F, F, F, F) {
    let t47138 = F::cast_from(0.43374325201206959368e-1_f64) * t3869 * t39538;
    let t47140 = F::cast_from(0.12842595503380418954e1_f64) * t3869 * t39427;
    let t47142 = F::cast_from(0.38025319932552508021e2_f64) * t3869 * t39535;
    let t47152 = F::new(120.0) * t3857 * t3853;
    let t47171 = t73 * t9940;
    let t47194 = t820 * t9991 * t843;
    let t47198 = t2482 * t1386 * t2237;
    (t47138, t47140, t47142, t47152, t47171, t47194, t47198)
}
