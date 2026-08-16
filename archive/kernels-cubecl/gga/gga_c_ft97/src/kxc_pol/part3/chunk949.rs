//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 949/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk949<F: Float>(t18659: F, t242: F, t3859: F, t3977: F, t729: F, t1175: F, t3746: F, t724: F, t13886: F, t13885: F, t1131: F, t2567: F) -> (F, F, F, F, F) {
    let t18660 = t242 * t18659;
    let t18664 = t729 * t3977 * t3859;
    let t18668 = t724 * t1175 * t3746;
    let t18671 = t13886 * t3859;
    let t18672 = t13885 * t18671;
    let t18675 = t2567 * t1131;
    (t18660, t18664, t18668, t18672, t18675)
}
