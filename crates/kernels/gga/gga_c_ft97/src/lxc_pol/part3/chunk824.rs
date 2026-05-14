//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 824/887 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk824<F: Float>(t5053: F, t713: F, t2574: F, t265: F, t766: F, t729: F, t762: F, t3842: F, t3977: F, t1175: F, t3837: F, t13927: F, t3864: F, t242: F, t3859: F, t3746: F, t724: F) -> (F, F, F, F, F, F, F, F) {
    let t18641 = t5053 * t713;
    let t18643 = t2574 * t265 * t18641;
    let t18646 = t5053 * t766;
    let t18648 = t729 * t762 * t18646;
    let t18652 = t729 * t3977 * t3842;
    let t18656 = t2574 * t1175 * t3837;
    let t18659 = t13927 * t3864;
    let t18660 = t242 * t18659;
    let t18664 = t729 * t3977 * t3859;
    let t18668 = t724 * t1175 * t3746;
    (t18643, t18648, t18652, t18656, t18659, t18660, t18664, t18668)
}
