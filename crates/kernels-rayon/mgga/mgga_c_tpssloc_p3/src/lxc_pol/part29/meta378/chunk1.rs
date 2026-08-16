//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 1512/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1512(t1597: f64, t4509: f64, t10237: f64, t10189: f64, t344: f64, t4343: f64, t2986: f64, t134: f64, t2978: f64, t4338: f64, t10190: f64, t4514: f64) -> (f64, f64, f64, f64) {
    let t13769 = t4509 * t1597;
    let t13770 = t13769 * t10237;
    let t13779 = t10189 * t344;
    let t13780 = t13779 * t4343;
    let t13782 = 0.37037037037037037036e-3_f64 * t2986 * t13780;
    let t13783 = t134 * t2978;
    let t13784 = t13783 * t344;
    let t13785 = t13784 * t4338;
    let t13787 = 0.24691358024691358024e-3_f64 * t2986 * t13785;
    let t13788 = t10190 * t4514;
    (t13770, t13782, t13787, t13788)
}
