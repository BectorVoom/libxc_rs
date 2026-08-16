//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1407/1464 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1407<F: Float>(t38474: F, t38475: F, t38481: F, t38489: F, t38512: F, t38522: F, t38524: F, t38535: F, t38541: F, t38559: F, t38573: F, t38588: F, t38607: F, t38643: F, t38648: F, t38654: F, t38663: F, t38664: F, t38678: F, t38719: F, t38738: F, t38762: F, t38769: F, t38773: F, t38787: F, t38801: F, t38811: F, t38812: F, t38824: F, t38836: F, t38850: F, t38863: F, t502: F) -> F {
    let t38869 = t502 * (t38863 + t38850 + t38836 + t38824 + t38811 + t38812 + t38801 + t38787 + t38773 + t38769 + t38762 + t38738 + t38719 + t38678 + t38663 + t38664 + t38654 + t38648 + t38643 + t38607 + t38588 + t38573 + t38559 + t38541 + t38535 + t38524 + t38522 + t38512 + t38489 + t38481 + t38474 + t38475);
    t38869
}
