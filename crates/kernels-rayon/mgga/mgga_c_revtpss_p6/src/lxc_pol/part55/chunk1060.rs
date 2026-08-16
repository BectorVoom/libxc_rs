//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1060/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1060(t32677: f64, t8707: f64, t32287: f64, t32266: f64, t32270: f64, t1444: f64, t8708: f64, t32250: f64, t1032: f64, t2097: f64, t1955: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t32678 = t8707 * t32677;
    let t32681 = 0.17354086964223805049e-2_f64 * t32287;
    let t32682 = 0.3718732920905101082e-4_f64 * t32266;
    let t32683 = 0.66119071333692697238e-4_f64 * t32270;
    let t32685 = t8708 * t1444;
    let t32686 = t32250 * t32685;
    let t32689 = t2097 * t1032;
    let t32690 = t1955 * t32689;
    (t32678, t32681, t32682, t32683, t32685, t32686, t32689, t32690)
}
