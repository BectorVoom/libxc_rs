//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1048/1170 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1048<F: Float>(t33: F, t892: F, t11064: F, t1955: F, t7283: F, t13846: F, t1941: F, t241: F, t25981: F, t820: F, t2022: F, t3999: F, t197: F, t530: F, t2013: F, t8995: F) -> (F, F, F, F, F, F, F, F) {
    let t27763 = t892 * t33;
    let t27799 = t11064 * t33;
    let t27868 = t1955 * t7283;
    let t27932 = t1941 * t13846;
    let t27940 = t820 * t25981 * t241;
    let t27980 = t3999 * t2022;
    let t28166 = t197 * t530;
    let t28167 = t2013 * t28166;
    let t28196 = t2013 * t8995;
    (t27763, t27799, t27868, t27932, t27940, t27980, t28167, t28196)
}
