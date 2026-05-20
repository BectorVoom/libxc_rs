//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 1079/1203 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk1079<F: Float>(t26916: F, t8937: F, t124664: F, t7642: F, t44546: F, t8946: F, t8947: F, t26894: F, t97312: F, t33494: F, t97081: F, t1269: F, t33468: F) -> (F, F, F, F, F, F) {
    let t124928 = t8937 * t26916;
    let t124931 = t7642 * t124664;
    let t124942 = F::cast_from(0.41319254676723345357e-4_f64) * t8946 * t8947 * t44546;
    let t124945 = t26894 * t97312;
    let t124950 = t97081 * t33494;
    let t124959 = t33468 * t1269 * t33494;
    (t124928, t124931, t124942, t124945, t124950, t124959)
}
