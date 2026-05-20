//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1006/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1006<F: Float>(t12948: F, t3610: F, t1263: F, t3584: F, t1122: F, t1042: F, t1260: F, t3666: F, t3172: F, t3713: F, t3711: F, t127: F, t3661: F, t371: F) -> (F, F, F, F, F) {
    let t12949 = t3610 * t12948;
    let t12951 = t1263 * t3584;
    let t12952 = t12951 * t1122;
    let t12953 = t1042 * t12952;
    let t12956 = t3666 * t1260;
    let t12959 = t3172 * t3713;
    let t12960 = t3711 * t12959;
    let t12963 = t371 * t127 * t3661;
    (t12949, t12953, t12956, t12960, t12963)
}
