//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 916/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk916<F: Float>(t13153: F, t3251: F, t4752: F, t13023: F, t2103: F, t4673: F, t1445: F, t43213: F, t833: F, t43217: F, t13136: F, t2197: F) -> (F, F, F, F, F) {
    let t43627 = F::new(0.28600391961480341335e1) * t13153 * t4752 * t3251;
    let t43630 = F::new(0.47667319935800568892e0) * t2103 * t4673 * t13023;
    let t43636 = F::new(0.11502877786176224903e2) * t833 * t1445 * t43213;
    let t43640 = F::new(0.11502877786176224903e2) * t833 * t1445 * t43217;
    let t43645 = F::new(0.11502877786176224903e2) * t2197 * t13136;
    (t43627, t43630, t43636, t43640, t43645)
}
