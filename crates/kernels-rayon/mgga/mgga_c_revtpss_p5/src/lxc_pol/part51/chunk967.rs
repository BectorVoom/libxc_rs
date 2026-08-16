//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 967/1200 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk967(t32255: f64, t32257: f64, t2022: f64, t7274: f64, t8707: f64, t25875: f64, t8590: f64, t1381: f64, t31805: f64, t555: f64) -> (f64, f64, f64, f64, f64) {
    let t32258 = t32255 * t32257;
    let t32262 = t8707 * t2022 * t7274;
    let t32265 = t25875 * t8590;
    let t32266 = t32265 * t1381;
    let t32267 = 0.1859366460452550541e-4_f64 * t32266;
    let t32268 = t31805 * t555;
    (t32258, t32262, t32265, t32267, t32268)
}
