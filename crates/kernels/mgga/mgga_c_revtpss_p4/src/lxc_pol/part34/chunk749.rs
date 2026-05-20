//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 749/1341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk749<F: Float>(t1647: F, t1652: F, t1696: F, t1978: F, t1983: F, t1986: F, t342: F, t7102: F, t7140: F, t7144: F, t7151: F, t7159: F, t7167: F, t7812: F, t7818: F, t7822: F, t7825: F, t7829: F, t7833: F, t7837: F) -> F {
    let t7840 = F::cast_from(0.65854491829355115987e0_f64) * t1647 * t1978 - F::cast_from(0.65854491829355115987e0_f64) * t7102 * t1652 + F::cast_from(0.65854491829355115987e0_f64) * t342 * t7812 - F::cast_from(0.65854491829355115987e0_f64) * t7140 * t1696 - F::cast_from(0.8673628188205199462e0_f64) * t7144 * t7818 + F::cast_from(0.8673628188205199462e0_f64) * t7151 * t7822 - F::cast_from(0.4336814094102599731e0_f64) * t7825 * t1986 + F::cast_from(0.8673628188205199462e0_f64) * t7159 * t7829 - F::cast_from(0.4336814094102599731e0_f64) * t7167 * t7833 - F::cast_from(0.4336814094102599731e0_f64) * t1983 * t7837;
    t7840
}
