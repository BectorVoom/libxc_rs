//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1182/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1182<F: Float>(t34879: F, t34893: F, t34895: F, t34897: F, t30782: F, t34883: F, t34887: F, t34891: F, t34901: F, t34905: F, t34909: F, t34913: F, t34916: F, t34920: F, t34923: F, t34926: F, t34929: F, t34933: F) -> F {
    let t37287 = F::cast_from(0.85748036236139473944e-3_f64) * t34879;
    let t37291 = F::new(0.3361875e0) * t34893;
    let t37292 = F::new(0.3361875e0) * t34895;
    let t37293 = F::cast_from(0.13073958333333333333e0_f64) * t34897;
    let t37305 = t37287 - F::new(0.4584375e-1) * t34883 - F::new(0.916875e-1) * t34887 - F::new(0.4584375e-1) * t34891 + t37291 + t37292 - t37293 + F::new(0.4584375e0) * t34901 - t34905 / F::new(8.0) - F::new(0.183375e0) * t34909 - F::new(0.916875e-1) * t34913 - F::new(0.916875e-1) * t34916 - F::new(0.916875e-1) * t34920 - F::new(0.916875e-1) * t34923 - F::new(0.183375e0) * t34926 - F::new(0.916875e-1) * t34929 - F::new(0.183375e0) * t30782 + F::cast_from(0.85748036236139473944e-3_f64) * t34933;
    t37305
}
