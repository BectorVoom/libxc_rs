//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1202/1286 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk1202<F: Float>(t1250: F, t20932: F, t17353: F, t5052: F, t17661: F, t5406: F, t1794: F, t3617: F, t372: F, t5047: F, t3603: F, t5284: F, t5332: F, t3720: F, t12866: F, t17340: F, t17342: F, t17693: F, t17729: F, t20914: F, t20917: F, t20923: F, t20927: F, t20929: F, t3711: F, t5340: F) -> (F,) {
    let t20933 = t1250 * t20932;
    let t20934 = t17353 * t20933;
    let t20937 = t1250 * t5052;
    let t20938 = t17353 * t20937;
    let t20941 = t17661 * t5406;
    let t20944 = t3617 * t1794;
    let t20945 = t372 * t20944;
    let t20946 = t1250 * t5047;
    let t20947 = t20945 * t20946;
    let t20950 = t3603 * t5284;
    let t20951 = t5332 * t20950;
    let t20952 = t3720 * t20951;
    let t20955 = 0.28582678745379824648e-3 * t3711 * t20914 + 0.28582678745379824648e-3 * t20917 + 0.5081365110289746604e-3 * t17340 - 0.95275595817932748827e-4 * t17342 - 0.47637797908966374413e-3 * t17729 * t20923 - 0.28582678745379824648e-3 * t20927 + 0.28582678745379824648e-3 * t12866 * t20929 + 0.28582678745379824648e-3 * t12866 * t20934 - 0.57165357490759649296e-3 * t17693 * t20938 + 0.28582678745379824648e-3 * t12866 * t20941 + 0.47637797908966374413e-3 * t17693 * t20947 + 0.85748036236139473944e-3 * t5340 * t20952;
    (t20955,)
}
