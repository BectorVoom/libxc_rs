//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 1008/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk1008<F: Float>(t10846: F, t10850: F, t10854: F, t10857: F, t10864: F, t10867: F, t11817: F, t11819: F, t11822: F, t11826: F, t11831: F, t1584: F, t3597: F) -> (F, F) {
    let t11833 = F::new(0.23804984598836975486e-2) * t11817 + F::new(0.54878743191129263322e-1) * t11819 + F::new(0.65495539973149862688e-2) * t11822 + F::new(0.65495539973149862688e-2) * t11826 - F::new(0.23287303101564395623e-1) * t10846 - F::new(0.69861909304693186869e-1) * t10850 - t10854 - F::new(0.48787202696913915093e-2) * t10857 + F::new(0.21831846657716620896e-2) * t11831 + t10864 + t10867;
    let t11835 = t1584 * t3597;
    (t11833, t11835)
}
