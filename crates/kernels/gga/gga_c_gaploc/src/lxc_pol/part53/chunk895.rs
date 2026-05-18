//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 895/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk895<F: Float>(t1897: F, t28957: F, t2936: F, t3266: F, t8942: F, t2508: F, t32658: F, t954: F, t40746: F, t40750: F, t40752: F, t40758: F) -> (F, F, F, F, F, F, F) {
    let t43131 = F::new(0.23071578690426672851e-1) * t1897 * t2936 * t28957;
    let t43143 = F::new(0.76905262301422242837e-2) * t1897 * t3266 * t8942;
    let t43146 = F::new(0.15381052460284448567e-1) * t2508 * t954 * t32658;
    let t43148 = F::new(0.64087718584518535698e-3) * t40746;
    let t43152 = F::new(0.64087718584518535698e-3) * t40750;
    let t43156 = F::new(0.64087718584518535698e-3) * t40752;
    let t43157 = F::new(0.64087718584518535698e-3) * t40758;
    (t43131, t43143, t43146, t43148, t43152, t43156, t43157)
}
