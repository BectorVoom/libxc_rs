//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 568/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk568(t14227: f64, t14252: f64, t2020: f64, t3180: f64, t2019: f64, t2604: f64, t3188: f64, t14375: f64, t2080: f64, t2211: f64, t739: f64, t14108: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t14689 = 0.1276937996798935182e-4_f64 * t14227;
    let t14693 = 0.23268647941669485538e-4_f64 * t14252;
    let t14696 = t2020 * t3180;
    let t14697 = t2019 * t14696;
    let t14701 = t2604 * t3188;
    let t14702 = 0.14967802127329760705e-1_f64 * t14701;
    let t14709 = 0.1276937996798935182e-4_f64 * t14375;
    let t14710 = t2211 * t2080;
    let t14711 = t739 * t14710;
    let t14712 = 0.2993560425465952141e-1_f64 * t14711;
    let t14865 = 0.15965655602485078085e0_f64 * t14108;
    (t14689, t14693, t14696, t14697, t14702, t14709, t14710, t14712, t14865)
}
