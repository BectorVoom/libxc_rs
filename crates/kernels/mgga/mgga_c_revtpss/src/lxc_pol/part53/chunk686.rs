//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 686/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk686<F: Float>(t1000: F, t1097: F, t1978: F, t1983: F, t1986: F, t342: F, t7102: F, t7137: F, t7140: F, t7144: F, t7147: F, t7151: F, t7153: F, t7156: F, t7159: F, t7162: F, t7167: F, t7170: F, t7174: F, t989: F) -> F {
    let t7177 = F::new(0.65854491829355115987e0) * t989 * t1978 - F::new(0.65854491829355115987e0) * t7102 * t1000 + F::new(0.65854491829355115987e0) * t342 * t7137 - F::new(0.65854491829355115987e0) * t7140 * t1097 - F::new(0.8673628188205199462e0) * t7144 * t7147 + F::new(0.8673628188205199462e0) * t7151 * t7153 - F::new(0.4336814094102599731e0) * t7156 * t1986 + F::new(0.8673628188205199462e0) * t7159 * t7162 - F::new(0.4336814094102599731e0) * t7167 * t7170 - F::new(0.4336814094102599731e0) * t1983 * t7174;
    t7177
}
