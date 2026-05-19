//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 618/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk618<F: Float>(t4802: F, t1215: F, t1314: F, t457: F, t470: F, t4664: F, t4754: F, t4756: F, t4780: F, t4784: F, t4786: F, t4790: F, t4792: F, t4795: F, t4797: F, t4799: F) -> (F, F, F, F, F) {
    let t4803 = F::cast_from(0.51947267698127589897e2_f64) * t4802;
    let t4805 = t1215 * t1314 * t457;
    let t4806 = t470 * t4805;
    let t4807 = F::cast_from(0.35089340384731224426e1_f64) * t4806;
    let t4808 = t4754 + t4756 + t4664 + t4780 - t4784 - t4786 - t4790 - t4792 - t4795 + t4797 - t4799 - t4803 + t4807;
    (t4803, t4805, t4806, t4807, t4808)
}
