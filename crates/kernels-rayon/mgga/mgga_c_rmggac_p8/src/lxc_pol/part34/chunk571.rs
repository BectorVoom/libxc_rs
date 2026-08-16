//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 571/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk571(t14043: f64, t14049: f64, t14054: f64, t14057: f64, t14060: f64, t3230: f64, t504: f64, t14094: f64, t22: f64, t2227: f64, t656: f64, t2145: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t14602 = 0.4379826523225341797e-6_f64 * t14043;
    let t14603 = 0.87596530464506835935e-6_f64 * t14049;
    let t14607 = 0.19709219354514038085e-5_f64 * t14054;
    let t14608 = 0.87596530464506835935e-6_f64 * t14057;
    let t14609 = 0.2627895913935205078e-5_f64 * t14060;
    let t14611 = t504 * t3230;
    let t14612 = 0.19957069503106347607e-1_f64 * t14611;
    let t14616 = 0.10227998120342003148e-1_f64 * t14094;
    let t14617 = t2227 * t22;
    let t14618 = t14617 * t656;
    let t14619 = t2145 * t14618;
    (t14602, t14603, t14607, t14608, t14609, t14612, t14616, t14617, t14618, t14619)
}
