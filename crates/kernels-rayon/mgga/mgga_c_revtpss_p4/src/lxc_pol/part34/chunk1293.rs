//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1293/1341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1293(t113563: f64, t113600: f64, t113634: f64, t113667: f64, t100535: f64, t107286: f64, t107358: f64, t1646: f64, t1651: f64, t1695: f64, t1976: f64, t225: f64, t23958: f64, t25699: f64, t27419: f64, t27568: f64, t27621: f64, t29728: f64, t29759: f64, t29807: f64, t29826: f64, t29835: f64, t29848: f64, t29871: f64, t29876: f64, t29884: f64, t342: f64, t355: f64, t357: f64, t385: f64, t6251: f64, t7144: f64, t7145: f64, t7151: f64, t7159: f64, t7160: f64, t7829: f64, t7837: f64, t93436: f64, t93497: f64) -> (f64, f64) {
    let t113669 = t113563 + t113600 + t113634 + t113667;
    let t113678 = -0.26020884564615598386e1_f64 * t29835 * t7837 + 0.39512695097613069591e1_f64 * t27568 * t6251 + 0.26020884564615598386e1_f64 * t27419 * t29884 + 0.52041769129231196772e1_f64 * t27419 * t29728 + 0.26020884564615598386e1_f64 * t7151 * t7145 * t29807 * t1651 + 0.15612530738769359031e2_f64 * t25699 * t7160 * t29871 * t1695 + 0.52041769129231196772e1_f64 * t107286 * t7829 - 0.13010442282307799193e1_f64 * t27621 * t29826 - 0.26020884564615598386e1_f64 * t100535 * t29848 + 0.10408353825846239354e2_f64 * t93436 * t29759 * t355 * t357 * t1646 - 0.10408353825846239354e2_f64 * t93497 * t29759 * t355 * t357 * t1651 + 0.26020884564615598386e1_f64 * t7159 * t7160 * t29807 * t1695 - 0.26020884564615598386e1_f64 * t107358 * t29876 + 0.65854491829355115987e0_f64 * t342 * t113669 * t225 * t385 - 0.8673628188205199462e0_f64 * t7144 * t7145 * t1976 * t23958;
    (t113669, t113678)
}
