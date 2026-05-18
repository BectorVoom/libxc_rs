//! GGA_C_GAPC lxc pol — lxc_pol part 33 (v4rho2sigma2_12) CSE chunk 745/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part33_v4rho2sigma2_12_chunk745<F: Float>(t185: F, t8769: F, t1717: F, t8726: F, t8729: F, t8732: F, t8735: F, t8739: F, t8741: F, t8746: F, t8752: F, t8756: F, t8758: F, t8761: F, t8766: F) -> (F, F) {
    let t8770 = t185 * t8769;
    let t8771 = t8770 * t1717;
    let t8773 = F::new(0.30362304687500000001e-3) * t8726 - F::new(0.14492726735651760868e-5) * t8729 - F::new(0.14492726735651760868e-5) * t8732 - F::new(0.72463633678258804342e-6) * t8735 + F::new(0.14492726735651760868e-5) * t8739 + F::new(0.28985453471303521736e-5) * t8741 + F::new(0.25745714186718600947e-6) * t8746 - F::new(0.33199136135672468897e-7) * t8752 + F::new(0.59028064049225649701e-7) * t8756 - F::new(0.28985453471303521736e-5) * t8758 + F::new(0.29518907335069444446e-5) * t8761 - F::new(0.33765185592488808582e-6) * t8766 - F::new(0.77294542590142724635e-6) * t8771;
    (t8770, t8773)
}
