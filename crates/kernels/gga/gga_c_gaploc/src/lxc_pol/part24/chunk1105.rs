//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1105/1439 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1105<F: Float>(t28737: F, t9797: F, t2586: F, t2679: F, t9796: F, t2013: F, t9813: F, t825: F, t826: F, t9829: F, t15362: F, t9810: F) -> (F, F, F, F, F) {
    let t28738 = t28737 * t9797;
    let t28739 = F::new(0.1533717038156829987e1) * t28738;
    let t28742 = t9796 * t2586 * t2679;
    let t28743 = F::new(0.1533717038156829987e1) * t28742;
    let t28792 = t2013 * t9813;
    let t28793 = F::new(0.1022478025437886658e1) * t28792;
    let t28795 = t825 * t826 * t9829;
    let t28796 = F::new(0.1022478025437886658e1) * t28795;
    let t28800 = F::new(0.11916829983950142223e0) * t15362 * t9810;
    (t28739, t28743, t28793, t28796, t28800)
}
