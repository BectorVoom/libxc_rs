//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2295/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2295<F: Float>(t26215: F, t81228: F, t81326: F, t16436: F, t1985: F, t6889: F, t6906: F, t2015: F, t40590: F, t6897: F, t6907: F, t90544: F) -> (F, F, F, F) {
    let t90686 = t81228 * t81326 * t26215;
    let t90687 = F::cast_from(0.16449340668482264365e-1_f64) * t90686;
    let t90690 = t1985 * t6889 * t6906 * t16436;
    let t90696 = t40590 * t2015;
    let t90701 = t6897 * t90544 * t6907;
    (t90687, t90690, t90696, t90701)
}
