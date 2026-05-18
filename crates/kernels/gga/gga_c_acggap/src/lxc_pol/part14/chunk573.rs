//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 573/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk573<F: Float>(t1008: F, t1581: F, t4797: F, t4799: F, t4808: F, t4816: F, t3228: F, t532: F, t1569: F, t3670: F, t542: F, t537: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t4846 = F::new(0.85748036236139473944e-3) * t1008 * t1581;
    let t4856 = t4797 / F::new(6.0);
    let t4857 = F::new(2.0) / F::new(3.0) * t4799;
    let t4860 = t4808 / F::new(12.0);
    let t4863 = F::new(4.0) / F::new(3.0) * t4816;
    let t4881 = t3228 * t532;
    let t4884 = F::new(0.17149607247227894789e-2) * t1008 * t1569;
    let t4889 = t3670 * t532;
    let t4891 = t3670 * t542;
    let t4897 = t3670 * t537;
    (t4846, t4856, t4857, t4860, t4863, t4881, t4884, t4889, t4891, t4897)
}
