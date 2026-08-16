//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1038/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1038<F: Float>(t21498: F, t21529: F, t21560: F, t21612: F, t383: F, t1625: F, t5866: F, t1060: F, t1615: F, t1932: F, t360: F, t5936: F) -> (F, F, F, F) {
    let t21614 = t21498 + t21529 + t21560 + t21612;
    let t21615 = t383 * t21614;
    let t21617 = t1625 * t5866;
    let t21618 = t21617 * t1060;
    let t21622 = t1932 * t1615 * t360;
    let t21623 = t5936 * t21622;
    (t21614, t21615, t21618, t21623)
}
