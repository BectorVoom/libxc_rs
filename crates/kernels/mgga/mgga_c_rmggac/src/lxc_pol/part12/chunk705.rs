//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 705/1088 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk705<F: Float>(t8356: F, t8467: F, t8470: F, t8477: F, t8484: F, t8488: F, t8492: F, t8534: F, t8657: F, t8820: F, t9037: F, t9069: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t9730 = F::cast_from(0.30487649791575028314e-3_f64) * t8356;
    let t9743 = F::cast_from(0.72042316457491791906e-3_f64) * t8467;
    let t9744 = F::cast_from(0.10248087766267884742e-3_f64) * t8470;
    let t9758 = F::cast_from(0.30487649791575028314e-3_f64) * t8477;
    let t9759 = F::cast_from(0.43368970657079495312e-4_f64) * t8484;
    let t9760 = F::cast_from(0.30487649791575028314e-3_f64) * t8488;
    let t9761 = F::cast_from(0.43368970657079495312e-4_f64) * t8492;
    let t9768 = F::cast_from(0.18183107769496894486e-1_f64) * t8534;
    let t9947 = F::cast_from(0.18183107769496894486e-1_f64) * t8657;
    let t10035 = F::new(2.0) * t8820;
    let t10060 = F::cast_from(0.24829349937757072982e-4_f64) * t9037;
    let t10061 = F::new(0.4726e1) * t9069;
    (t9730, t9743, t9744, t9758, t9759, t9760, t9761, t9768, t9947, t10035, t10060, t10061)
}
