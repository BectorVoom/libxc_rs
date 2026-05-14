//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 795/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk795<F: Float>(t687: F, t11869: F, t2795: F, t286: F, t244: F, t2974: F, t813: F, t224: F, t2827: F, t2627: F, t883: F, t273: F, t2787: F, t791: F, t709: F, t804: F) -> (F, F, F, F, F, F, F, F) {
    let t11870 = t687 * t687;
    let t11874 = 0.12304822629859687989e5 * t286 * t11869 * t11870 * t2795;
    let t11878 = t2974 * t244;
    let t11882 = t813 * t813;
    let t11883 = 1.0 / t11882;
    let t11889 = t224 * t2827;
    let t11893 = t883 * t2627;
    let t11898 = 0.46785788981077169656e1 * t286 * t791 * t2787 * t273;
    let t11900 = 120.0 * t709 * t804;
    (t11870, t11874, t11878, t11883, t11889, t11893, t11898, t11900)
}
