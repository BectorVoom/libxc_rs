//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1059/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1059<F: Float>(t33953: F, t4241: F, t13364: F, t34833: F, t13299: F, t2001: F, t4344: F, t4349: F, t7741: F, t2290: F, t7630: F, t1549: F, t30540: F) -> (F, F, F, F, F, F) {
    let t34834 = t33953 * t4241;
    let t34836 = t34833 * t13364 * t34834;
    let t34839 = t34833 * t13299 * t34834;
    let t34841 = t2001 * t4344;
    let t34844 = t7741 * t4349;
    let t34849 = t7630 * t2290;
    let t34851 = t30540 * t1549;
    (t34836, t34839, t34841, t34844, t34849, t34851)
}
