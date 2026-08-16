//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1770/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1770<F: Float>(t59: F, t9971: F, t23040: F, t2617: F, t23061: F, t6604: F, t1891: F, t1895: F, t213: F, t39041: F, t1887: F, t206: F, t80845: F) -> (F, F, F, F, F) {
    let t81816 = t9971 * t59;
    let t81824 = t2617 * t23040;
    let t81835 = t23061 * t6604;
    let t81849 = t39041 * t1891 * t213 * t1895;
    let t81852 = t80845 * t206 * t1887;
    (t81816, t81824, t81835, t81849, t81852)
}
