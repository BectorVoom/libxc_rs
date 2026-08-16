//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1333/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1333(t10314: f64, t20592: f64, t6710: f64, t34246: f64, t6717: f64, t6914: f64, t10513: f64, t20441: f64, t10532: f64, t4529: f64, t579: f64, t30903: f64) -> (f64, f64, f64, f64, f64) {
    let t34749 = 0.30674340763136599742e2_f64 * t6710 * t20592 * t10314;
    let t34752 = 0.62115540045351614476e2_f64 * t6914 * t6717 * t34246;
    let t34762 = 0.1656414401209376386e3_f64 * t6914 * t20441 * t10513;
    let t34766 = 0.73618417831527839379e2_f64 * t10532 * t579 * t4529 * t10513;
    let t34773 = 0.63904876589867916128e-1_f64 * t30903;
    (t34749, t34752, t34762, t34766, t34773)
}
