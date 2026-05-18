//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1293/1341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1293<F: Float>(t113563: F, t113600: F, t113634: F, t113667: F, t100535: F, t107286: F, t107358: F, t1646: F, t1651: F, t1695: F, t1976: F, t225: F, t23958: F, t25699: F, t27419: F, t27568: F, t27621: F, t29728: F, t29759: F, t29807: F, t29826: F, t29835: F, t29848: F, t29871: F, t29876: F, t29884: F, t342: F, t355: F, t357: F, t385: F, t6251: F, t7144: F, t7145: F, t7151: F, t7159: F, t7160: F, t7829: F, t7837: F, t93436: F, t93497: F) -> (F, F) {
    let t113669 = t113563 + t113600 + t113634 + t113667;
    let t113678 = -F::new(0.26020884564615598386e1) * t29835 * t7837 + F::new(0.39512695097613069591e1) * t27568 * t6251 + F::new(0.26020884564615598386e1) * t27419 * t29884 + F::new(0.52041769129231196772e1) * t27419 * t29728 + F::new(0.26020884564615598386e1) * t7151 * t7145 * t29807 * t1651 + F::new(0.15612530738769359031e2) * t25699 * t7160 * t29871 * t1695 + F::new(0.52041769129231196772e1) * t107286 * t7829 - F::new(0.13010442282307799193e1) * t27621 * t29826 - F::new(0.26020884564615598386e1) * t100535 * t29848 + F::new(0.10408353825846239354e2) * t93436 * t29759 * t355 * t357 * t1646 - F::new(0.10408353825846239354e2) * t93497 * t29759 * t355 * t357 * t1651 + F::new(0.26020884564615598386e1) * t7159 * t7160 * t29807 * t1695 - F::new(0.26020884564615598386e1) * t107358 * t29876 + F::new(0.65854491829355115987e0) * t342 * t113669 * t225 * t385 - F::new(0.8673628188205199462e0) * t7144 * t7145 * t1976 * t23958;
    (t113669, t113678)
}
