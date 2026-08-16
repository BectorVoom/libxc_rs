//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2555/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2555<F: Float>(t1099: F, t1118: F, t71558: F, t71571: F, t71585: F, t71597: F, t71611: F, t71624: F, t71636: F, t71649: F, t21813: F, t43964: F) -> (F, F) {
    let t71655 = F::cast_from(1.0_f64) * t1099 * (t71558 + t71571 + t71585 + t71597 + t71611 + t71624 + t71636 + t71649) * t1118;
    let t71657 = F::cast_from(0.51726012919273400301e3_f64) * t43964 * t21813;
    (t71655, t71657)
}
