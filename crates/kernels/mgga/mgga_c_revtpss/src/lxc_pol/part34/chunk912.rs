//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 912/1196 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk912<F: Float>(t4598: F, t6120: F, t4614: F, t11304: F, t15189: F, t18919: F, t18924: F, t18934: F, t23479: F, t23483: F, t23487: F, t23490: F, t23501: F, t23505: F, t916: F, t923: F) -> (F, F, F, F) {
    let t23521 = t4598 * t6120;
    let t23523 = t4614 * t6120;
    let t23535 = -t11304 - 4.0 / 9.0 * t15189 + 2.0 / 9.0 * t18919 - 2.0 / 3.0 * t18924 + t18934 / 3.0 - 10.0 / 27.0 * t23479 + 4.0 / 3.0 * t23483 - 2.0 / 3.0 * t23501 - 2.0 * t23487 + 2.0 * t23505 - t23490 / 3.0;
    let t23536 = t916 * t23535;
    let t23538 = t923 * t23535;
    (t23521, t23523, t23536, t23538)
}
