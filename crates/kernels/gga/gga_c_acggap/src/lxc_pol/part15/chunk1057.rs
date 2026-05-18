//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1057/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1057<F: Float>(t35774: F, t35784: F, t35788: F, t35794: F, t35797: F, t35799: F, t35814: F, t35816: F, t35827: F, t35837: F, t35844: F, t35848: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t37714 = F::new(0.62896184579208304136e-2) * t35774;
    let t37717 = F::new(0.68598428988911579156e-2) * t35784;
    let t37718 = F::new(0.25158473831683321655e-2) * t35788;
    let t37721 = F::new(0.94344276868812456204e-2) * t35794;
    let t37722 = F::new(0.85748036236139473944e-3) * t35797;
    let t37723 = F::new(0.68598428988911579156e-2) * t35799;
    let t37731 = F::new(0.16006300097412701803e-1) * t35814;
    let t37732 = F::new(0.42874018118069736972e-3) * t35816;
    let t37735 = F::new(0.28582678745379824648e-3) * t35827;
    let t37739 = F::new(0.25724410870841842184e-2) * t35837;
    let t37741 = F::new(0.42874018118069736972e-3) * t35844;
    let t37743 = F::new(0.16809375e0) * t35848;
    (t37714, t37717, t37718, t37721, t37722, t37723, t37731, t37732, t37735, t37739, t37741, t37743)
}
