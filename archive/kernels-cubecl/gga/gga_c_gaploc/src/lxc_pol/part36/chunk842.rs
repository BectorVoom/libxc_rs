//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 842/1029 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk842<F: Float>(t40182: F, t40184: F, t40187: F, t1445: F, t1450: F, t41813: F, t41814: F, t41818: F, t41820: F, t41822: F, t41829: F, t41831: F, t41834: F, t41837: F, t41841: F, t41844: F, t41846: F, t41847: F, t41848: F, t41849: F, t41850: F, t41851: F, t447: F) -> F {
    let t41852 = F::cast_from(0.25561950635947166451e0_f64) * t40182;
    let t41853 = F::cast_from(0.89376224879626066674e-1_f64) * t40184;
    let t41854 = F::cast_from(0.17875244975925213335e0_f64) * t40187;
    let t41855 = -t41813 - F::cast_from(0.13803453343411469884e2_f64) * t41814 - F::cast_from(0.13803453343411469884e2_f64) * t41818 + F::cast_from(0.47667319935800568892e0_f64) * t41820 - F::cast_from(0.23005755572352449806e1_f64) * t1450 * t1445 * t41822 * t447 - t41829 + t41831 + t41834 - t41837 - F::cast_from(0.14300195980740170668e1_f64) * t41841 + t41844 + t41846 + t41847 - t41848 + t41849 - t41850 - t41851 + t41852 + t41853 - t41854;
    t41855
}
