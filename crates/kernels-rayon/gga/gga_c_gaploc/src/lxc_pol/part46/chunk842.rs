//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 842/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk842(t40182: f64, t40184: f64, t40187: f64, t1445: f64, t1450: f64, t41813: f64, t41814: f64, t41818: f64, t41820: f64, t41822: f64, t41829: f64, t41831: f64, t41834: f64, t41837: f64, t41841: f64, t41844: f64, t41846: f64, t41847: f64, t41848: f64, t41849: f64, t41850: f64, t41851: f64, t447: f64) -> f64 {
    let t41852 = 0.25561950635947166451e0_f64 * t40182;
    let t41853 = 0.89376224879626066674e-1_f64 * t40184;
    let t41854 = 0.17875244975925213335e0_f64 * t40187;
    let t41855 = -t41813 - 0.13803453343411469884e2_f64 * t41814 - 0.13803453343411469884e2_f64 * t41818 + 0.47667319935800568892e0_f64 * t41820 - 0.23005755572352449806e1_f64 * t1450 * t1445 * t41822 * t447 - t41829 + t41831 + t41834 - t41837 - 0.14300195980740170668e1_f64 * t41841 + t41844 + t41846 + t41847 - t41848 + t41849 - t41850 - t41851 + t41852 + t41853 - t41854;
    t41855
}
