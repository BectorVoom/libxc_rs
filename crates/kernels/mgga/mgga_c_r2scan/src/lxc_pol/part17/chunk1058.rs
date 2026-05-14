//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1058/1120 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1058<F: Float>(t15059: F, t986: F, t3270: F, t3269: F, t10610: F, t3465: F, t42454: F, t42392: F, t1115: F, t2892: F, t36986: F, t3275: F, t3472: F, t42851: F, t11342: F, t43726: F) -> (F, F, F, F, F, F) {
    let t44011 = t15059 * t986;
    let t44012 = t3270 * t44011;
    let t44014 = t3269 * t44012 / 2.0;
    let t44017 = 3.0 / 2.0 * t10610 * t3465 * t42454;
    let t44020 = 3.0 * t10610 * t3465 * t42392;
    let t44022 = t3270 * t1115 * t2892;
    let t44024 = 3.0 / 2.0 * t36986 * t44022;
    let t44027 = 5.0 / 8.0 * t3275 * t3472 * t42851;
    let t44029 = 3.0 / 4.0 * t43726 * t11342;
    (t44014, t44017, t44020, t44024, t44027, t44029)
}
