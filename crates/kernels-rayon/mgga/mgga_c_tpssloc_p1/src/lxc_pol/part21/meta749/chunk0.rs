//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2621/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2621(t5181: f64, t591: f64, t16465: f64, t225: f64, t12344: f64, t5234: f64, t1369: f64, t16336: f64, t3876: f64, t16333: f64, t3866: f64, t1831: f64, t40284: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t53852 = 16.0_f64 * t5181 * t591;
    let t53866 = t16465 * t225;
    let t53880 = t5234 * t12344;
    let t53881 = t53880 * t1369;
    let t53883 = t16336 * t3876;
    let t53893 = t3866 * t16333;
    let t53895 = t40284 * t1831;
    (t53852, t53866, t53880, t53881, t53883, t53893, t53895)
}
