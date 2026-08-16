//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1400/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1400(t10513: f64, t20441: f64, t6914: f64, t10532: f64, t4529: f64, t579: f64, t30903: f64, t30907: f64, t30923: f64, t30927: f64, t10417: f64, t1397: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t34762 = 0.1656414401209376386e3_f64 * t6914 * t20441 * t10513;
    let t34766 = 0.73618417831527839379e2_f64 * t10532 * t579 * t4529 * t10513;
    let t34773 = 0.63904876589867916128e-1_f64 * t30903;
    let t34774 = 0.95857314884801874192e-1_f64 * t30907;
    let t34775 = 0.31952438294933958064e-1_f64 * t30923;
    let t34776 = 0.12780975317973583226e0_f64 * t30927;
    let t34777 = t1397 * t10417;
    (t34762, t34766, t34773, t34774, t34775, t34776, t34777)
}
