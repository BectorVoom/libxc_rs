//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 1993/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1993<F: Float>(t2690: F, t6612: F, t812: F, t831: F, t59: F, t9971: F, t23040: F, t2617: F, t23061: F, t6604: F, t1891: F, t1895: F, t213: F, t39041: F) -> (F, F, F, F, F, F) {
    let t81807 = t812 * t6612 * t2690;
    let t81808 = t81807 * t831;
    let t81816 = t9971 * t59;
    let t81824 = t2617 * t23040;
    let t81835 = t23061 * t6604;
    let t81849 = t39041 * t1891 * t213 * t1895;
    (t81807, t81808, t81816, t81824, t81835, t81849)
}
