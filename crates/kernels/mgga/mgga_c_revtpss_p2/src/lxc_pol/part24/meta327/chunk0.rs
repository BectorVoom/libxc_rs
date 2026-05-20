//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1136/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1136<F: Float>(t22213: F, t13666: F, t13668: F, t13670: F, t13887: F, t9524: F, t9542: F, t9588: F, t9598: F, t9854: F, t9857: F, t9865: F, t9868: F) -> (F, F, F, F, F, F) {
    let t22928 = F::cast_from(0.17544670867903938621e1_f64) * t22213;
    let t22929 = F::cast_from(0.32530743900905219526e-1_f64) * t13666;
    let t22930 = F::new(36.0) * t13668;
    let t22931 = F::new(96.0) * t13670;
    let t22932 = F::cast_from(0.73245789224026180216e-3_f64) * t13887;
    let t22933 = -t9588 - t9524 + t9598 - t22928 + t22929 + t22930 + t22931 + t9542 - t9854 - t9857 + t9865 + t9868 + t22932;
    (t22928, t22929, t22930, t22931, t22932, t22933)
}
