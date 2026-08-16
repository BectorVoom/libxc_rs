//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 945/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk945<F: Float>(t41965: F, t6717: F, t6914: F, t10532: F, t10533: F, t10557: F, t30936: F, t1: F, t2392: F, t3338: F, t544: F, t594: F) -> (F, F, F, F) {
    let t42242 = F::cast_from(0.62115540045351614476e2_f64) * t6914 * t6717 * t41965;
    let t42245 = F::cast_from(0.27606906686822939767e2_f64) * t10532 * t10533 * t41965;
    let t42250 = F::cast_from(0.17875244975925213335e2_f64) * t10557 * t30936;
    let t42254 = t544 * t594 * t3338 * t1 * t2392;
    (t42242, t42245, t42250, t42254)
}
