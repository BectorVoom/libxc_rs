//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1064/1093 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1064<F: Float>(t2097: F, t22857: F, t102364: F, t109539: F, t109553: F, t109555: F, t109567: F, t109579: F, t109609: F, t115107: F, t22975: F, t23043: F, t26079: F, t27837: F, t28899: F, t30283: F, t30309: F, t4003: F, t543: F, t6896: F, t7295: F, t7301: F, t7511: F, t96374: F) -> (F, F) {
    let t115166 = t2097 * t22857;
    let t115181 = 0.43368140941025997312e-1 * t109539 + t96374 - 0.43368140941025997312e-1 * t109553 + 0.77108554593144223218e-1 * t109555 - 0.86736281882051994623e-1 * t109567 + 0.52041769129231196772e1 * t27837 * t30283 + 0.29272321618148349057e-1 * t109579 + 0.21684070470512998656e-1 * t109609 - 0.65854491829355115987e0 * t7511 * t23043 - 0.68549505033305214441e-2 * t102364 + 0.4336814094102599731e0 * t7295 * t7301 * t115166 * t543 - 0.26020884564615598386e1 * t7295 * t26079 * t115107 * t4003 + 0.26020884564615598386e1 * t27837 * t30309 - 0.39512695097613069591e1 * t7511 * t22975 + 0.39512695097613069591e1 * t28899 * t6896;
    (t115166, t115181)
}
