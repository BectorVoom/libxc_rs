//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1205/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1205(t35774: f64, t35784: f64, t35788: f64, t35790: f64, t35794: f64, t35797: f64, t35799: f64, t32866: f64, t32867: f64, t35766: f64, t35768: f64, t35772: f64, t35778: f64, t35782: f64, t35792: f64, t35801: f64, t35804: f64, t35808: f64) -> f64 {
    let t37714 = 0.62896184579208304136e-2_f64 * t35774;
    let t37717 = 0.68598428988911579156e-2_f64 * t35784;
    let t37718 = 0.25158473831683321655e-2_f64 * t35788;
    let t37719 = 0.17149607247227894789e-2_f64 * t35790;
    let t37721 = 0.94344276868812456204e-2_f64 * t35794;
    let t37722 = 0.85748036236139473944e-3_f64 * t35797;
    let t37723 = 0.68598428988911579156e-2_f64 * t35799;
    let t37727 = 0.13719685797782315831e-1_f64 * t35766 - 0.13719685797782315831e-1_f64 * t35768 - t32866 - t32867 + 0.12862205435420921092e-2_f64 * t35772 + t37714 + 0.42874018118069736972e-2_f64 * t35778 + 0.25724410870841842184e-2_f64 * t35782 + t37717 + t37718 + t37719 - 0.17149607247227894789e-1_f64 * t35792 - t37721 + t37722 + t37723 - 0.41159057393346947494e-1_f64 * t35801 + 0.94344276868812456208e-2_f64 * t35804 - 0.37737710747524982482e-2_f64 * t35808;
    t37727
}
