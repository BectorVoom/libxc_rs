//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 681/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk681<F: Float>(t10491: F, t309: F, t2399: F, t865: F, t89: F, t1882: F, t2864: F, t2850: F, t2889: F, t10394: F, t10276: F, t2787: F, t458: F) -> (F, F, F, F, F, F, F, F) {
    let t10492 = t10491 * t309;
    let t10514 = t89 * t2399 * t865;
    let t10533 = t1882 * t2864;
    let t10539 = t1882 * t2850;
    let t10545 = t1882 * t2889;
    let t10552 = t10394 / F::new(3.0);
    let t10555 = F::new(2.0) / F::new(3.0) * t10276;
    let t10559 = t458 * t2787;
    (t10492, t10514, t10533, t10539, t10545, t10552, t10555, t10559)
}
