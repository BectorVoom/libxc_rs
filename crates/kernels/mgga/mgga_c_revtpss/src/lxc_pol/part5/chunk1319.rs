//! MGGA_C_REVTPSS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1319/1422 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part5_v3rho3_2_chunk1319<F: Float>(t20945: F, t20946: F, t3603: F, t5284: F, t5332: F, t3720: F, t12866: F, t17340: F, t17342: F, t17693: F, t17729: F, t20914: F, t20917: F, t20923: F, t20927: F, t20929: F, t20934: F, t20938: F, t20941: F, t3711: F, t5340: F) -> F {
    let t20947 = t20945 * t20946;
    let t20950 = t3603 * t5284;
    let t20951 = t5332 * t20950;
    let t20952 = t3720 * t20951;
    let t20955 = F::new(0.28582678745379824648e-3) * t3711 * t20914 + F::new(0.28582678745379824648e-3) * t20917 + F::new(0.5081365110289746604e-3) * t17340 - F::new(0.95275595817932748827e-4) * t17342 - F::new(0.47637797908966374413e-3) * t17729 * t20923 - F::new(0.28582678745379824648e-3) * t20927 + F::new(0.28582678745379824648e-3) * t12866 * t20929 + F::new(0.28582678745379824648e-3) * t12866 * t20934 - F::new(0.57165357490759649296e-3) * t17693 * t20938 + F::new(0.28582678745379824648e-3) * t12866 * t20941 + F::new(0.47637797908966374413e-3) * t17693 * t20947 + F::new(0.85748036236139473944e-3) * t5340 * t20952;
    t20955
}
