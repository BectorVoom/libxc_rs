//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1120/1304 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1120<F: Float>(t7301: F, t8087: F, t7300: F, t1720: F, t8882: F, t32428: F, t8034: F, t32432: F, t8039: F, t1729: F, t8878: F, t1742: F, t493: F) -> (F, F, F, F, F, F, F) {
    let t34250 = t7301 * t8087;
    let t34251 = t7300 * t34250;
    let t34254 = t1720 * t8882;
    let t34260 = t8034 * t32428;
    let t34263 = t32432 * t8039;
    let t34266 = t1729 * t8878;
    let t34271 = t493 * t1742;
    (t34250, t34251, t34254, t34260, t34263, t34266, t34271)
}
