//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 923/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk923<F: Float>(t1065: F, t481: F, t3270: F, t10667: F, t2104: F, t3436: F, t2302: F) -> (F, F, F) {
    let t10668 = t1065 * t481;
    let t10669 = t3270 * t10668;
    let t10670 = t10667 * t10669;
    let t10671 = F::new(3.0) / F::new(2.0) * t10670;
    let t10672 = t2104 * t3436;
    let t10673 = t2302 * t10672;
    (t10669, t10671, t10673)
}
