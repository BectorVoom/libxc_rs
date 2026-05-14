//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 1063/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk1063<F: Float>(t1040: F, t3687: F, t8863: F, t3115: F, t436: F, t8780: F, t3144: F, t34372: F, t11458: F, t1936: F, t19670: F, t11326: F, t9262: F, t3688: F, t8877: F, t26102: F, t3709: F) -> (F, F, F, F, F, F, F) {
    let t34573 = t8863 * t3687 * t1040;
    let t34576 = t3115 * t436 * t8780;
    let t34582 = t34372 * t3144;
    let t34585 = t19670 * t1936 * t11458;
    let t34587 = t11326 * t9262;
    let t34589 = t3688 * t8877;
    let t34591 = t3709 * t26102;
    (t34573, t34576, t34582, t34585, t34587, t34589, t34591)
}
