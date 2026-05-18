//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 760/1468 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk760<F: Float>(t2555: F, t7187: F, t2549: F, t2559: F, t481: F, t685: F, t729: F, t1897: F, t2508: F, t2542: F, t5269: F, t7129: F, t7137: F, t7141: F, t7144: F, t7147: F, t7152: F, t7158: F, t7161: F, t7165: F, t7168: F, t7170: F, t7175: F, t7179: F, t7182: F, t7184: F) -> (F, F, F, F) {
    let t7188 = t7187 * t2555;
    let t7190 = t2549 * t2559;
    let t7193 = t481 * t729 * t685;
    let t7194 = t7193 * t2555;
    let t7196 = F::new(0.99692006687028833308e-3) * t7141 - F::new(0.46143157380853345702e-1) * t2508 * t7144 + F::new(0.76905262301422242837e-2) * t2508 * t7147 - F::new(0.46143157380853345702e-1) * t7129 * t2542 + F::new(0.15381052460284448567e-1) * t5269 * t7152 - F::new(0.61524209841137794271e-1) * t7137 * t2542 + F::new(0.92286314761706691403e-1) * t2508 * t7158 - F::new(0.53833683610995569986e-1) * t2508 * t7161 - F::new(0.15381052460284448567e-1) * t1897 * t7165 + F::new(0.85450291446024714264e-3) * t7168 + F::new(0.46143157380853345702e-1) * t1897 * t7170 - F::new(0.32043859292259267849e-3) * t7175 - F::new(0.32043859292259267849e-3) * t7179 + F::new(0.32043859292259267849e-3) * t7182 - F::new(0.85450291446024714264e-3) * t7184 - F::new(0.64087718584518535698e-3) * t7188 + F::new(0.64087718584518535698e-3) * t7190 + F::new(0.85450291446024714264e-3) * t7194;
    (t7188, t7190, t7194, t7196)
}
