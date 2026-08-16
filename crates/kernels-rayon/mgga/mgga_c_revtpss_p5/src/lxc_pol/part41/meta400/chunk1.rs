//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 1363/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1363(t20945: f64, t20946: f64, t3603: f64, t5284: f64, t5332: f64, t3720: f64, t12866: f64, t17340: f64, t17342: f64, t17693: f64, t17729: f64, t20914: f64, t20917: f64, t20923: f64, t20927: f64, t20929: f64, t20934: f64, t20938: f64, t20941: f64, t3711: f64, t5340: f64) -> f64 {
    let t20947 = t20945 * t20946;
    let t20950 = t3603 * t5284;
    let t20951 = t5332 * t20950;
    let t20952 = t3720 * t20951;
    let t20955 = 0.28582678745379824648e-3_f64 * t3711 * t20914 + 0.28582678745379824648e-3_f64 * t20917 + 0.5081365110289746604e-3_f64 * t17340 - 0.95275595817932748827e-4_f64 * t17342 - 0.47637797908966374413e-3_f64 * t17729 * t20923 - 0.28582678745379824648e-3_f64 * t20927 + 0.28582678745379824648e-3_f64 * t12866 * t20929 + 0.28582678745379824648e-3_f64 * t12866 * t20934 - 0.57165357490759649296e-3_f64 * t17693 * t20938 + 0.28582678745379824648e-3_f64 * t12866 * t20941 + 0.47637797908966374413e-3_f64 * t17693 * t20947 + 0.85748036236139473944e-3_f64 * t5340 * t20952;
    t20955
}
