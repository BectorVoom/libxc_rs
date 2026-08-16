//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 883/1013 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk883(t11622: f64, t1445: f64, t2530: f64, t813: f64, t43925: f64, t43927: f64, t43930: f64, t2365: f64, t35500: f64, t7390: f64, t36798: f64, t787: f64, t9824: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t45792 = 0.46011511144704899612e1_f64 * t813 * t1445 * t11622 * t2530;
    let t45793 = 0.17875244975925213335e0_f64 * t43925;
    let t45794 = 0.17875244975925213335e0_f64 * t43927;
    let t45795 = 0.19171462976960374838e0_f64 * t43930;
    let t45797 = t7390 * t2365 * t35500;
    let t45798 = 0.14896037479937677779e-1_f64 * t45797;
    let t45800 = t787 * t36798 * t9824;
    (t45792, t45793, t45794, t45795, t45798, t45800)
}
