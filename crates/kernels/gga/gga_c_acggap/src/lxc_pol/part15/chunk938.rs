//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 938/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk938<F: Float>(t35623: F, t35631: F, t35646: F, t35672: F, t35678: F, t35682: F, t35685: F, t35702: F, t35709: F, t35736: F, t35747: F, t35755: F, t35774: F, t35784: F, t35788: F, t35794: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t37636 = 0.12579236915841660828e-2 * t35623;
    let t37639 = 0.18868855373762491241e-2 * t35631;
    let t37646 = 0.305625e-1 * t35646;
    let t37658 = 0.13719685797782315831e-1 * t35672;
    let t37661 = 0.13719685797782315831e-1 * t35678;
    let t37663 = 0.57165357490759649296e-3 * t35682;
    let t37665 = 11.0 / 24.0 * t35685;
    let t37672 = 0.18868855373762491241e-2 * t35702;
    let t37675 = 0.64025200389650807212e-1 * t35709;
    let t37696 = 0.68598428988911579156e-2 * t35736;
    let t37701 = 0.85748036236139473944e-3 * t35747;
    let t37704 = 0.34299214494455789578e-1 * t35755;
    let t37714 = 0.62896184579208304136e-2 * t35774;
    let t37717 = 0.68598428988911579156e-2 * t35784;
    let t37718 = 0.25158473831683321655e-2 * t35788;
    let t37721 = 0.94344276868812456204e-2 * t35794;
    (t37636, t37639, t37646, t37658, t37661, t37663, t37665, t37672, t37675, t37696, t37701, t37704, t37714, t37717, t37718, t37721)
}
