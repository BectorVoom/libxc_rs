//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2589/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2589<F: Float>(t51470: F, t51472: F, t51474: F, t51476: F, t51478: F, t51480: F, t51482: F, t51485: F, t51549: F, t51593: F, t51831: F, t51833: F, t51835: F, t51839: F, t51844: F, t51847: F, t51851: F, t51853: F, t51855: F, t51857: F) -> F {
    let t52453 = t51470 - t51472 + t51474 - t51476 + t51478 - t51480 + t51482 - t51485 - t51831 + t51833 - t51549 - t51593 - t51835 - t51839 - t51844 + t51847 + t51851 + t51853 + t51855 + t51857;
    t52453
}
