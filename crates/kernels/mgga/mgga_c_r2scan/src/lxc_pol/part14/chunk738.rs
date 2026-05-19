//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 738/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk738<F: Float>(t5954: F, t637: F, t1763: F, t1942: F, t1762: F, t1835: F, t377: F, t1946: F, t1767: F, t1987: F, t424: F, t625: F) -> (F, F, F, F, F) {
    let t5955 = t5954 * t637;
    let t5957 = t1763 * t1942;
    let t5959 = F::cast_from(0.32530743900905219526e-1_f64) * t1762 * t5957;
    let t5960 = t377 * t1835;
    let t5961 = t5960 * t1946;
    let t5963 = F::cast_from(0.28895839882605942646e1_f64) * t1762 * t5961;
    let t5964 = t1767 * t1987;
    let t5966 = F::cast_from(0.96319466275353142157e0_f64) * t1762 * t5964;
    let t5967 = t424 * t625;
    (t5955, t5959, t5963, t5966, t5967)
}
