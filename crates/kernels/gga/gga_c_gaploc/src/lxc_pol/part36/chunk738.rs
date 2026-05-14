//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 738/884 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk738<F: Float>(t41838: F, t447: F, t6963: F, t6964: F, t2877: F, t9490: F, t9494: F, t40167: F, t40170: F, t40172: F, t40176: F, t40178: F, t40182: F, t40184: F, t40187: F, t1445: F, t1450: F, t41813: F, t41814: F, t41818: F, t41820: F, t41822: F, t41829: F, t41831: F, t41834: F, t41837: F) -> (F, F) {
    let t41839 = t41838 * t447;
    let t41841 = t6963 * t6964 * t41839;
    let t41844 = 0.35750489951850426669e0 * t9490 * t2877;
    let t41846 = 0.35750489951850426669e0 * t9494 * t2877;
    let t41847 = 0.3575048995185042667e0 * t40167;
    let t41848 = 0.17875244975925213335e0 * t40170;
    let t41849 = 0.19171462976960374838e1 * t40172;
    let t41850 = 0.42603251059911944084e0 * t40176;
    let t41851 = 0.11502877786176224903e1 * t40178;
    let t41852 = 0.25561950635947166451e0 * t40182;
    let t41853 = 0.89376224879626066674e-1 * t40184;
    let t41854 = 0.17875244975925213335e0 * t40187;
    let t41855 = -t41813 - 0.13803453343411469884e2 * t41814 - 0.13803453343411469884e2 * t41818 + 0.47667319935800568892e0 * t41820 - 0.23005755572352449806e1 * t1450 * t1445 * t41822 * t447 - t41829 + t41831 + t41834 - t41837 - 0.14300195980740170668e1 * t41841 + t41844 + t41846 + t41847 - t41848 + t41849 - t41850 - t41851 + t41852 + t41853 - t41854;
    (t41839, t41855)
}
