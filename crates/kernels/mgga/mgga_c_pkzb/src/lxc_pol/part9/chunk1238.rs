//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1238/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1238<F: Float>(t2104: F, t5974: F, t7677: F, t2899: F, t7682: F, t2922: F, t7775: F, t2918: F, t5939: F, t757: F, t2082: F, t2939: F) -> (F, F, F, F, F) {
    let t21637 = t2104 * t5974 * t7677;
    let t21640 = t2899 * t5974 * t7682;
    let t21643 = t2922 * t5974 * t7775;
    let t21651 = t757 * t5939 * t2918;
    let t21652 = F::new(0.14291339372689912324e-3) * t21651;
    let t21655 = t2082 * t2939;
    (t21637, t21640, t21643, t21652, t21655)
}
